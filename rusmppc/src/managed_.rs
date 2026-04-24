// This implementation in based on `bb8` pool.
// The public api is relatively stable.
// It should be possible to swap out the internal implementation without breaking the public api.

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

use crate::{Client, ConnectionBuilder, error::Error as RusmppcError, event::EventChannel};

const TARGET: &str = "rusmppc::managed::client";

/// TODO: docs
#[derive(Debug)]
pub enum ManagedEvent<E> {
    /// TODO: docs
    Connected,
    /// TODO: docs
    Bound,
    /// TODO: docs
    Disconnected,
    /// TODO: docs
    Event(E),
}

/// TODO: docs
#[derive(Clone)]
pub struct ManagedClient {
    inner: Arc<ManagedClientInner>,
    // Used to tell the reconnecting background task to stop when the client is dropped.
    _watch: watch::Receiver<()>,
}

impl Debug for ManagedClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagedClient").finish()
    }
}

struct ManagedClientInner {
    creator: Box<dyn BoundClientCreator>,
    client: RwLock<Client>,
}

impl ManagedClientInner {
    fn new(creator: Box<dyn BoundClientCreator>, client: Client) -> Self {
        Self {
            creator,
            client: RwLock::new(client),
        }
    }

    async fn get(&self) -> Result<RwLockReadGuard<'_, Client>, RusmppcError> {
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

impl ManagedClient {
    fn new(inner: Arc<ManagedClientInner>, watch: watch::Receiver<()>) -> Self {
        Self {
            inner,
            _watch: watch,
        }
    }

    // TODO: in order for this to make sense, we have to add the backoff and stuff
    /// TODO: docs
    pub async fn get(&self) -> Result<Client, RusmppcError> {
        // TODO: we handle timeouts here, because we might want to use retries while establishing the connection.

        self.inner.get().await.map(|client| client.clone())
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
pub struct UnboundManagedConnectionBuilder<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> UnboundManagedConnectionBuilder<E> {
    pub(crate) fn new(builder: ConnectionBuilder<E>) -> Self {
        Self { builder }
    }

    /// TODO: docs
    pub fn transmitter(self, bind: BindTransmitter) -> ManagedConnectionBuilder<E> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Transmitter(bind))
    }

    /// TODO: docs
    pub fn receiver(self, bind: BindReceiver) -> ManagedConnectionBuilder<E> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Receiver(bind))
    }

    /// TODO: docs
    pub fn transceiver(self, bind: BindTransceiver) -> ManagedConnectionBuilder<E> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Transceiver(bind))
    }

    /// TODO: docs
    pub fn unbound(self) -> ManagedConnectionBuilder<E> {
        ManagedConnectionBuilder::new(self.builder, BindMode::None)
    }
}

#[derive(Debug)]
pub struct ManagedConnectionBuilder<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
    bind: BindMode,
    auto_reconnect_interval: Option<Duration>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManagedConnectionBuilder<E> {
    fn new(builder: ConnectionBuilder<E>, bind: BindMode) -> Self {
        Self {
            builder,
            bind,
            auto_reconnect_interval: Some(Duration::from_secs(5)),
        }
    }

    /// TODO: docs
    pub fn auto_reconnect_interval(mut self, auto_reconnect_interval: Duration) -> Self {
        self.auto_reconnect_interval = Some(auto_reconnect_interval);
        self
    }

    /// TODO: docs
    pub fn no_auto_reconnect_interval(mut self) -> Self {
        self.auto_reconnect_interval = None;
        self
    }

    /// TODO: docs
    pub fn with_auto_reconnect_interval(
        mut self,
        auto_reconnect_interval: Option<Duration>,
    ) -> Self {
        self.auto_reconnect_interval = auto_reconnect_interval;
        self
    }
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManagedConnectionBuilder<E>
where
    E::Event: Send + Sync + 'static,
{
    async fn run(
        self,
        connect: Connect,
    ) -> Result<
        (
            ManagedClient,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        RusmppcError,
    > {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let rx = UnboundedReceiverStream::new(rx);

        let creator = BoundClientCreatorImpl::new(self.builder, connect, self.bind, tx);
        let client = creator.connect().await?;
        let client = Arc::new(ManagedClientInner::new(Box::new(creator), client));

        let (w_tx, w_rx) = watch::channel(());

        if let Some(interval) = self.auto_reconnect_interval {
            let client_c = client.clone();

            tokio::spawn(async move {
                tracing::trace!(target: TARGET, ?interval, "Starting reconnect task");

                loop {
                    tokio::select! {
                        _ = w_tx.closed() => {
                            tracing::debug!(target: TARGET, "Stopping reconnect task");

                            break;
                        }
                        _ = tokio::time::sleep(interval) => {
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

    /// TODO: docs
    pub async fn connect_fn<F, Fut, S>(
        self,
        f: F,
    ) -> Result<
        (
            ManagedClient,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        RusmppcError,
    >
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<S, std::io::Error>> + Send + 'static,
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        self.run(Connect::Connector(Box::new(f))).await
    }

    /// TODO: docs
    pub async fn connect(
        self,
        url: impl Into<String>,
    ) -> Result<
        (
            ManagedClient,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        RusmppcError,
    > {
        self.run(Connect::Url(url.into())).await
    }
}

enum Connect {
    Url(String),
    Connector(Box<dyn Connector>),
}

struct BoundClientCreatorImpl<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
    connect: Connect,
    bind: BindMode,
    tx: UnboundedSender<ManagedEvent<E::Event>>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> BoundClientCreatorImpl<E>
where
    E::Event: Send + Sync + 'static,
{
    fn new(
        builder: ConnectionBuilder<E>,
        connect: Connect,
        bind: BindMode,
        tx: UnboundedSender<ManagedEvent<E::Event>>,
    ) -> Self {
        Self {
            builder,
            connect,
            bind,
            tx,
        }
    }

    async fn connect(&self) -> Result<Client, RusmppcError> {
        tracing::debug!(target: TARGET, "Connecting");

        let (client, mut events) = match self.connect {
            Connect::Url(ref url) => self
                .builder
                .clone()
                .connect(url)
                .await
                .map(|(client, events)| (client, EventStream::new_a(events)))?,
            Connect::Connector(ref connector) => connector
                .connect()
                .await
                .map_err(RusmppcError::Connect)
                .map(|stream| self.builder.clone().connected(stream))
                .map(|(client, events)| (client, EventStream::new_b(events)))?,
        };

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

        tokio::spawn(async move {
            while let Some(event) = events.next().await {
                let _ = tx.send(ManagedEvent::Event(event));
            }

            let _ = tx.send(ManagedEvent::Disconnected);

            tracing::warn!(target: TARGET, "Disconnected");
        });

        Ok(client)
    }
}

trait BoundClientCreator: Send + Sync + 'static {
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Client, RusmppcError>> + Send + '_>>;
}

impl<E: EventChannel + Clone + Send + Sync + 'static> BoundClientCreator
    for BoundClientCreatorImpl<E>
where
    E::Event: Send + Sync + 'static,
{
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Client, RusmppcError>> + Send + '_>> {
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
