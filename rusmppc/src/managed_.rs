use std::{
    fmt::Debug,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use futures::{Stream, StreamExt};
use rusmpp::pdus::{BindReceiver, BindTransceiver, BindTransmitter};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{RwLock, RwLockReadGuard, mpsc::UnboundedSender, watch},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tryhard::backoff_strategies::{
    BackoffStrategy, ExponentialBackoff, FixedBackoff, LinearBackoff, NoBackoff,
};

use crate::{
    Client, ConnectionBuilder,
    error::Error,
    event_::EventChannel,
    runtime_::{Delay, Timeout, tokio::Tokio},
};

const TARGET: &str = "rusmppc::managed::client";

/// Events emitted by the [`ManagedClient`].
#[derive(Debug)]
pub enum ManagedEvent<E> {
    /// Emitted when the client is connected to the server.
    Connected,
    /// Emitted when the client is successfully bound to the server.
    Bound,
    /// Emitted when the client is disconnected from the server.
    Disconnected,
    /// Emitted when the client receives an event from the server.
    Event(E),
}

/// A managed `SMPP` client that automatically handles reconnection and binding.
pub struct ManagedClient<T = Tokio> {
    inner: Arc<ManagedClientInner<T>>,
    // Used to tell the reconnecting background task to stop when the client is dropped.
    _watch: watch::Receiver<()>,
    _t: std::marker::PhantomData<T>,
}

impl<T> Clone for ManagedClient<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _watch: self._watch.clone(),
            _t: std::marker::PhantomData,
        }
    }
}

impl<T> Debug for ManagedClient<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagedClient").finish()
    }
}

struct ManagedClientInner<T = Tokio> {
    creator: Box<dyn BoundClientCreator<T>>,
    client: RwLock<Client<T>>,
}

impl<T: Timeout> ManagedClientInner<T> {
    fn new(creator: Box<dyn BoundClientCreator<T>>, client: Client<T>) -> Self {
        Self {
            creator,
            client: RwLock::new(client),
        }
    }

    async fn get(&self) -> Result<RwLockReadGuard<'_, Client<T>>, Error>
    where
        T: 'static,
    {
        {
            let client = self.client.read().await;

            if client.is_active() {
                return Ok(client);
            }
        }

        let mut client = self.client.write().await;

        *client = self.creator.connect().await?;

        Ok(client.downgrade())
    }
}

impl<T: Timeout> ManagedClient<T> {
    fn new(inner: Arc<ManagedClientInner<T>>, watch: watch::Receiver<()>) -> Self {
        Self {
            inner,
            _watch: watch,
            _t: std::marker::PhantomData,
        }
    }

    /// Gets a connected and bound [`Client`].
    ///
    /// This method will block until a connected [`Client`] is available, and will automatically attempt to reconnect if the connection is lost.
    pub async fn get(&self) -> Result<Client<T>, Error>
    where
        T: 'static,
    {
        self.inner.get().await.map(|client| client.clone())
    }

    // TODO: we want to have the same api like `Client`: client.timeout().get()

    /// Gets a connected and bound [`Client`] with a timeout.
    pub async fn get_with_timeout(&self, timeout: Duration) -> Option<Result<Client<T>, Error>>
    where
        T: 'static,
    {
        T::timeout(timeout, self.get()).await
    }
}

#[derive(Debug, Clone)]
enum BindMode {
    None,
    Transmitter(BindTransmitter),
    Receiver(BindReceiver),
    Transceiver(BindTransceiver),
}

impl BindMode {
    const fn is_bind(&self) -> bool {
        !matches!(self, BindMode::None)
    }
}

#[derive(Debug)]
pub struct UnboundManagedConnectionBuilder<
    E: EventChannel + Clone + Send + Sync + 'static,
    D: Delay,
    T: Timeout,
    R,
> {
    builder: ConnectionBuilder<E, D, T, R>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static, D: Delay, T: Timeout, R>
    UnboundManagedConnectionBuilder<E, D, T, R>
{
    pub(crate) fn new(builder: ConnectionBuilder<E, D, T, R>) -> Self {
        Self { builder }
    }

    /// Binds the [`ManagedClient`] as a transmitter.
    ///
    /// Every time the client reconnects, it will automatically bind as a transmitter using the provided [`BindTransmitter`].
    pub fn transmitter(self, bind: BindTransmitter) -> ManagedConnectionBuilder<E, D, T, R> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Transmitter(bind))
    }

    /// Binds the [`ManagedClient`] as a receiver.
    ///
    /// Every time the client reconnects, it will automatically bind as a receiver using the provided [`BindReceiver`].
    pub fn receiver(self, bind: BindReceiver) -> ManagedConnectionBuilder<E, D, T, R> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Receiver(bind))
    }

    /// Binds the [`ManagedClient`] as a transceiver.
    ///
    /// Every time the client reconnects, it will automatically bind as a transceiver using the provided [`BindTransceiver`].
    pub fn transceiver(self, bind: BindTransceiver) -> ManagedConnectionBuilder<E, D, T, R> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Transceiver(bind))
    }

    /// Does not bind the [`ManagedClient`].
    ///
    /// Every time the client reconnects, it will not automatically bind.
    pub fn unbound(self) -> ManagedConnectionBuilder<E, D, T, R> {
        ManagedConnectionBuilder::new(self.builder, BindMode::None)
    }
}

#[derive(Debug)]
pub struct ManagedConnectionBuilder<
    E: EventChannel + Clone + Send + Sync + 'static,
    D: Delay,
    T: Timeout,
    R,
> {
    builder: ConnectionBuilder<E, D, T, R>,
    bind: BindMode,
    auto_reconnect_interval: Option<Duration>,
    max_delay: Option<Duration>,
    back_off: BackOff,
    max_retries: u32,
}

impl<E: EventChannel + Clone + Send + Sync + 'static, D: Delay, T: Timeout, R>
    ManagedConnectionBuilder<E, D, T, R>
{
    fn new(builder: ConnectionBuilder<E, D, T, R>, bind: BindMode) -> Self {
        Self {
            builder,
            bind,
            auto_reconnect_interval: Some(Duration::from_secs(5)),
            max_delay: None,
            back_off: BackOff::Exponential(ExponentialBackoff::new(Duration::from_secs(2))),
            max_retries: 10,
        }
    }

    /// Sets the interval at which the client will automatically attempt to reconnect.
    pub fn auto_reconnect_interval(mut self, auto_reconnect_interval: Duration) -> Self {
        self.auto_reconnect_interval = Some(auto_reconnect_interval);
        self
    }

    /// Disables automatic reconnection.
    ///
    /// The client will not automatically attempt to reconnect if the connection is lost.
    ///
    /// You can still manually call [`ManagedClient::get`] to reconnect.
    pub fn no_auto_reconnect_interval(mut self) -> Self {
        self.auto_reconnect_interval = None;
        self
    }

    /// Sets the interval at which the client will automatically attempt to reconnect.
    pub fn with_auto_reconnect_interval(
        mut self,
        auto_reconnect_interval: Option<Duration>,
    ) -> Self {
        self.auto_reconnect_interval = auto_reconnect_interval;
        self
    }

    /// Sets the maximum delay between reconnection attempts.
    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = Some(delay);
        self
    }

    /// Disables the maximum delay between reconnection attempts.
    pub fn no_max_delay(mut self) -> Self {
        self.max_delay = None;
        self
    }

    /// Sets the maximum delay between reconnection attempts.
    pub fn with_max_delay(mut self, delay: Option<Duration>) -> Self {
        self.max_delay = delay;
        self
    }

    /// Disables backoff between reconnection attempts.
    pub fn no_backoff(mut self) -> Self {
        self.back_off = BackOff::None;
        self
    }

    /// Sets an exponential backoff for reconnection attempts.
    pub fn exponential_backoff(mut self, initial_delay: Duration) -> Self {
        self.back_off = BackOff::Exponential(ExponentialBackoff::new(initial_delay));
        self
    }

    /// Sets a fixed backoff for reconnection attempts.
    pub fn fixed_backoff(mut self, delay: Duration) -> Self {
        self.back_off = BackOff::Fixed(FixedBackoff::new(delay));
        self
    }

    /// Sets a linear backoff for reconnection attempts.
    pub fn linear_backoff(mut self, delay: Duration) -> Self {
        self.back_off = BackOff::Linear(LinearBackoff::new(delay));
        self
    }

    /// Sets a maximum number of reconnection attempts before giving up.
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
}

#[cfg(feature = "tokio")]
impl<E: EventChannel, D: Delay, T: Timeout> ManagedConnectionBuilder<E, D, T, Tokio>
where
    E: Clone + Send + Sync + 'static,
    E::Event: Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
    D::Future: Send,
    T: Clone + Send + Sync + 'static,
{
    async fn run(
        self,
        connect: Connect,
    ) -> Result<
        (
            ManagedClient<T>,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        Error,
    > {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let rx = UnboundedReceiverStream::new(rx);

        let creator = BoundClientCreatorImpl::new(
            self.builder,
            connect,
            self.bind,
            self.max_delay,
            self.back_off,
            self.max_retries,
            tx,
        );

        let client = creator.connect().await?;
        let client = Arc::new(ManagedClientInner::new(Box::new(creator), client));

        let (w_tx, w_rx) = watch::channel(());

        if let Some(interval) = self.auto_reconnect_interval {
            let client_c = client.clone();

            Tokio::spawn(async move {
                tracing::trace!(target: TARGET, ?interval, "Starting reconnect task");

                loop {
                    tokio::select! {
                        _ = w_tx.closed() => {
                            tracing::debug!(target: TARGET, "Stopping reconnect task");

                            break;
                        }
                        _ = D::delay(interval) => {
                            tracing::trace!(target: TARGET, "Triggering reconnection");

                            // Trigger a reconnection if the connection was closed

                            if let Err(err) = client_c.get().await {
                                tracing::error!(target: TARGET, ?err, "Failed to reconnect");
                            }
                        }
                    }
                }

                tracing::trace!(target: TARGET, "Reconnect task stopped");
            });
        }

        Ok((ManagedClient::new(client, w_rx), rx))
    }

    /// Sets a function to be called when connecting.
    ///
    /// See [`ConnectionBuilder::connected`] for more details.
    pub async fn connect_fn<F, Fut, S>(
        self,
        f: F,
    ) -> Result<
        (
            ManagedClient<T>,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        Error,
    >
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<S, std::io::Error>> + Send + 'static,
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        self.run(Connect::Connector(Box::new(f))).await
    }

    /// Connects to the `SMPP` server.
    ///
    /// See [`ConnectionBuilder::connect`] for more details.
    pub async fn connect(
        self,
        url: impl Into<String>,
    ) -> Result<
        (
            ManagedClient<T>,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        Error,
    > {
        self.run(Connect::Url(url.into())).await
    }
}

enum Connect {
    Url(String),
    Connector(Box<dyn Connector>),
}

struct BoundClientCreatorImpl<E: EventChannel, D: Delay, T: Timeout, R> {
    builder: ConnectionBuilder<E, D, T, R>,
    connect: Connect,
    bind: BindMode,
    max_delay: Option<Duration>,
    back_off: BackOff,
    max_retries: u32,
    tx: UnboundedSender<ManagedEvent<E::Event>>,
}

impl<E: EventChannel, D: Delay, T: Timeout, R> BoundClientCreatorImpl<E, D, T, R>
where
    E: Clone + Send + Sync + 'static,
    E::Event: Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
    D::Future: Send,
    T: Clone + 'static,
    R: Clone + 'static,
{
    fn new(
        builder: ConnectionBuilder<E, D, T, R>,
        connect: Connect,
        bind: BindMode,
        max_delay: Option<Duration>,
        back_off: BackOff,
        max_retries: u32,
        tx: UnboundedSender<ManagedEvent<E::Event>>,
    ) -> Self {
        Self {
            builder,
            connect,
            bind,
            max_delay,
            back_off,
            max_retries,
            tx,
        }
    }
}

#[cfg(feature = "tokio")]
impl<E: EventChannel, D: Delay, T: Timeout> BoundClientCreatorImpl<E, D, T, Tokio>
where
    E: Clone + Send + Sync + 'static,
    E::Event: Send + Sync + 'static,
    D: Clone + Send + Sync + 'static,
    D::Future: Send,
    T: Clone + 'static,
{
    async fn connect(&self) -> Result<Client<T>, Error> {
        tracing::debug!(target: TARGET, "Connecting");

        let connect = move || async move {
            match self.connect {
                Connect::Url(ref url) => self
                    .builder
                    .clone()
                    .connect(url)
                    .await
                    .map(|(client, events)| (client, EventStream::new_a(events))),
                Connect::Connector(ref connector) => connector
                    .connect()
                    .await
                    .map_err(Error::Connect)
                    .map(|stream| self.builder.clone().connected(stream))
                    .map(|(client, events)| (client, EventStream::new_b(events))),
            }
        };

        let max_delay = self.max_delay;
        let max_retries = self.max_retries;
        let mut fut = tryhard::retry_fn(connect)
            .retries(self.max_retries)
            .custom_backoff(self.back_off)
            .on_retry(|attempt, next_delay, _| async move {
                tracing::warn!(target: TARGET, ?attempt, ?max_retries, ?next_delay, ?max_delay, "Connection attempt failed");
            });

        if let Some(delay) = self.max_delay {
            fut = fut.max_delay(delay)
        };

        let (client, mut events) = fut.await?;

        let _ = self.tx.send(ManagedEvent::Connected);

        tracing::debug!(target: TARGET, "Connected");

        match self.bind.clone() {
            BindMode::Transmitter(bind) => {
                client.bind_transmitter(bind).await?;
            }
            BindMode::Receiver(bind) => {
                client.bind_receiver(bind).await?;
            }
            BindMode::Transceiver(bind) => {
                client.bind_transceiver(bind).await?;
            }
            BindMode::None => {}
        }

        if self.bind.is_bind() {
            let _ = self.tx.send(ManagedEvent::Bound);

            tracing::debug!(target: TARGET, "Bound");
        }

        let tx = self.tx.clone();

        Tokio::spawn(async move {
            while let Some(event) = events.next().await {
                let _ = tx.send(ManagedEvent::Event(event));
            }

            let _ = tx.send(ManagedEvent::Disconnected);

            tracing::warn!(target: TARGET, "Disconnected");
        });

        Ok(client)
    }
}

trait BoundClientCreator<T>: Send + Sync + 'static {
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Client<T>, Error>> + Send + '_>>;
}

impl<E: EventChannel, D: Delay, T: Timeout, R> BoundClientCreator<T>
    for BoundClientCreatorImpl<E, D, T, R>
where
    E: Clone + Send + Sync + 'static,
    E::Event: Send + Sync + 'static,
    D: Send + Sync + 'static,
    T: Send + Sync + 'static,
    R: Send + Sync + 'static,
{
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Client<T>, Error>> + Send + '_>> {
        Box::pin(async move { self.connect().await })
    }
}

trait UnpinAsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T: AsyncRead + AsyncWrite + Unpin + Send> UnpinAsyncReadWrite for T {}

#[allow(clippy::type_complexity)]
trait Connector: Send + Sync + 'static {
    fn connect(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn UnpinAsyncReadWrite>, std::io::Error>> + Send>>;
}

impl<F, Fut, S> Connector for F
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<S, std::io::Error>> + Send + 'static,
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    fn connect(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn UnpinAsyncReadWrite>, std::io::Error>> + Send>>
    {
        let fut = (self)();

        Box::pin(async move {
            let stream = fut.await?;

            Ok(Box::new(stream) as Box<dyn UnpinAsyncReadWrite>)
        })
    }
}

pin_project_lite::pin_project! {
    pub struct EventStream<A, B, E> {
        #[pin]
        stream: StreamOrStream<A, B>,
        _marker: std::marker::PhantomData<E>,
    }
}

impl<A, B, E> EventStream<A, B, E> {
    pub fn new_a(stream: A) -> Self {
        Self {
            stream: StreamOrStream::A { stream },
            _marker: std::marker::PhantomData,
        }
    }

    pub fn new_b(stream: B) -> Self {
        Self {
            stream: StreamOrStream::B { stream },
            _marker: std::marker::PhantomData,
        }
    }
}

pin_project_lite::pin_project! {
    #[project = StreamOrStreamProj]
    pub enum StreamOrStream<A, B> {
        A { #[pin] stream: A },
        B { #[pin] stream: B },
    }
}

impl<A, B, E> Stream for EventStream<A, B, E>
where
    A: Stream<Item = E>,
    B: Stream<Item = E>,
{
    type Item = E;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match this.stream.project() {
            StreamOrStreamProj::A { stream } => stream.poll_next(cx),
            StreamOrStreamProj::B { stream } => stream.poll_next(cx),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BackOff {
    None,
    Exponential(ExponentialBackoff),
    Fixed(FixedBackoff),
    Linear(LinearBackoff),
}

impl<'a, E> BackoffStrategy<'a, E> for BackOff {
    type Output = Duration;

    fn delay(&mut self, attempt: u32, error: &'a E) -> Duration {
        match self {
            BackOff::None => NoBackoff.delay(attempt, error),
            BackOff::Exponential(backoff) => backoff.delay(attempt, error),
            BackOff::Fixed(backoff) => backoff.delay(attempt, error),
            BackOff::Linear(backoff) => backoff.delay(attempt, error),
        }
    }
}

// TODO: tests
