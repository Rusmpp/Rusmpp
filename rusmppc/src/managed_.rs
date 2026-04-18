use std::{
    fmt::Debug,
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use bb8::{ManageConnection, Pool, RunError};
use futures::{Stream, StreamExt};
use rusmpp::pdus::{BindReceiver, BindTransceiver, BindTransmitter};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{mpsc::UnboundedSender, watch},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    Client, ConnectionBuilder,
    error::Error as RusmppcError,
    event::{DefaultEventChannel, EventChannel},
};

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
#[derive(Debug, thiserror::Error)]
pub enum ManagedError {
    /// TODO: docs
    #[error(transparent)]
    Error(RusmppcError),
    /// TODO: docs
    #[error("Attempted to get a client but the provided timeout was exceeded")]
    TimedOut,
}

impl From<RunError<RusmppcError>> for ManagedError {
    fn from(error: RunError<RusmppcError>) -> Self {
        match error {
            RunError::User(e) => Self::Error(e),
            RunError::TimedOut => Self::TimedOut,
        }
    }
}

/// TODO: docs
#[derive(Clone)]
pub struct ManagedClient<E: EventChannel + Clone + Send + Sync + 'static = DefaultEventChannel>
where
    E::Event: Send + Sync + 'static,
{
    pool: Pool<ClientConnectionManger<E>>,
    // Used to tell the reconnecting background task to stop when the client is dropped.
    _watch: watch::Receiver<()>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> Debug for ManagedClient<E>
where
    E::Event: Send + Sync + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagedClient").finish()
    }
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManagedClient<E>
where
    E::Event: Send + Sync + 'static,
{
    fn new(pool: Pool<ClientConnectionManger<E>>, watch: watch::Receiver<()>) -> Self {
        Self {
            pool,
            _watch: watch,
        }
    }

    /// Gets a connected [`Client`] from the managed connection.
    pub async fn get(&self) -> Result<impl Deref<Target = Client>, ManagedError> {
        let client = self.pool.get().await?;

        Ok(client)
    }

    /// Gets an owned connected [`Client`] from the managed connection.
    pub async fn get_owned(&self) -> Result<Client, ManagedError> {
        self.get().await.map(|client| client.deref().clone())
    }
}

#[derive(Debug, Clone)]
enum BindMode {
    None,
    Transmitter(BindTransmitter),
    Receiver(BindReceiver),
    Transceiver(BindTransceiver),
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

// TODO: we have to find a way to trigger a reconnection when the events stream is closed.
#[derive(Debug)]
pub struct ManagedConnectionBuilder<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
    connection_timeout: Duration,
    bind: BindMode,
    auto_reconnect_interval: Option<Duration>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManagedConnectionBuilder<E> {
    fn new(builder: ConnectionBuilder<E>, bind: BindMode) -> Self {
        Self {
            builder,
            connection_timeout: Duration::from_secs(30),
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

    /// Sets the connection timeout used by the managed connection.
    ///
    /// Clients returned by [`ManagedClient::get`] will wait this long before giving up and
    /// resolving with an error.
    ///
    /// Defaults to 30 seconds.
    ///
    /// # Panics
    ///
    /// Will panic if `connection_timeout` is 0.
    pub fn connection_timeout(mut self, connection_timeout: Duration) -> Self {
        assert!(
            connection_timeout > Duration::from_secs(0),
            "connection_timeout must be non-zero"
        );

        self.connection_timeout = connection_timeout;
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
            ManagedClient<E>,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        RusmppcError,
    > {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let rx = UnboundedReceiverStream::new(rx);

        let manager = ClientConnectionManger::new(self.builder, connect, self.bind, tx);

        // TODO: add the error sink so that we can pipe the connection errors to the managed event stream as well.
        let pool = bb8::Pool::builder()
            .max_size(1)
            .min_idle(1)
            .connection_timeout(self.connection_timeout)
            .idle_timeout(None)
            .max_lifetime(None)
            .build(manager)
            .await?;

        let (w_tx, w_rx) = watch::channel(());

        if let Some(interval) = self.auto_reconnect_interval {
            let pool_c = pool.clone();
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

                            pool_c.get().await.ok();
                        }
                    }
                }

                tracing::trace!(target: TARGET, "Reconnect task stopped");
            });
        }

        Ok((ManagedClient::new(pool, w_rx), rx))
    }

    /// TODO: docs
    pub async fn connect_fn<F, Fut, S>(
        self,
        f: F,
    ) -> Result<
        (
            ManagedClient<E>,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        RusmppcError,
    >
    where
        F: Fn() -> Fut + Clone + Send + Sync + 'static,
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
            ManagedClient<E>,
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

impl Clone for Connect {
    fn clone(&self) -> Self {
        match self {
            Connect::Url(url) => Connect::Url(url.clone()),
            Connect::Connector(connector) => Connect::Connector(connector.clone_box()),
        }
    }
}

impl Debug for Connect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connect::Url(url) => f.debug_tuple("Url").field(url).finish(),
            Connect::Connector(_) => f.debug_tuple("Connector").field(&"<connector>").finish(),
        }
    }
}

#[derive(Debug)]
struct ClientConnectionManger<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
    connect: Connect,
    bind: BindMode,
    tx: UnboundedSender<ManagedEvent<E::Event>>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ClientConnectionManger<E> {
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
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManageConnection for ClientConnectionManger<E>
where
    E::Event: Send + Sync + 'static,
{
    type Connection = Client;

    type Error = RusmppcError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        tracing::debug!(target: TARGET, "Connecting");

        let (client, mut events) = match self.connect.clone() {
            Connect::Url(url) => self
                .builder
                .clone()
                .connect(url)
                .await
                .map(|(client, events)| (client, EventStream::new_a(events)))?,
            Connect::Connector(connector) => connector
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

                let _ = self.tx.send(ManagedEvent::Bound);

                tracing::debug!(target: TARGET, "Bound");
            }
            BindMode::Receiver(bind) => {
                client.bind_receiver(bind).await?;

                let _ = self.tx.send(ManagedEvent::Bound);

                tracing::debug!(target: TARGET, "Bound");
            }
            BindMode::Transceiver(bind) => {
                client.bind_transceiver(bind).await?;

                let _ = self.tx.send(ManagedEvent::Bound);

                tracing::debug!(target: TARGET, "Bound");
            }
            BindMode::None => {}
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

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.is_active()
            .then_some(())
            .ok_or(RusmppcError::ConnectionClosed)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.is_closed()
    }
}

trait UnpinAsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T: AsyncRead + AsyncWrite + Unpin + Send> UnpinAsyncReadWrite for T {}

#[allow(clippy::type_complexity)]
trait Connector: Send + Sync + 'static {
    fn connect(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn UnpinAsyncReadWrite>, std::io::Error>> + Send>>;

    fn clone_box(&self) -> Box<dyn Connector>;
}

impl<F, Fut, S> Connector for F
where
    F: Fn() -> Fut + Clone + Send + Sync + 'static,
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

    fn clone_box(&self) -> Box<dyn Connector> {
        Box::new(self.clone())
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
