use std::{fmt::Debug, ops::Deref, time::Duration};

use bb8::{ManageConnection, Pool, RunError};
use futures::{Stream, StreamExt};
use rusmpp::pdus::{BindReceiver, BindTransceiver, BindTransmitter};
use tokio::sync::{mpsc::UnboundedSender, watch};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{Client, ConnectionBuilder, event::EventChannel};

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
    Error(crate::error::Error),
    /// TODO: docs
    #[error("Attempted to get a client but the provided timeout was exceeded")]
    TimedOut,
}

impl From<RunError<crate::error::Error>> for ManagedError {
    fn from(error: RunError<crate::error::Error>) -> Self {
        match error {
            RunError::User(e) => Self::Error(e),
            RunError::TimedOut => Self::TimedOut,
        }
    }
}

/// TODO: docs
#[derive(Clone)]
pub struct ManagedClient<E: EventChannel + Clone + Send + Sync + 'static>
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

    // TODO: this one takes an async function that returns a Result<AsyncRead + AsyncWrite, std::io::Error>
    /// TODO: docs
    pub async fn connected(self) {}

    /// TODO: docs
    pub async fn connect(
        self,
        url: impl Into<String>,
    ) -> Result<
        (
            ManagedClient<E>,
            impl Stream<Item = ManagedEvent<E::Event>> + Unpin + 'static,
        ),
        crate::error::Error,
    >
    where
        E::Event: Send + Sync + 'static,
    {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let rx = UnboundedReceiverStream::new(rx);

        let manager = ClientConnectionManger::new(self.builder, url.into(), self.bind, tx);

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
}

#[derive(Debug)]
struct ClientConnectionManger<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E>,
    url: String,
    bind: BindMode,
    tx: UnboundedSender<ManagedEvent<E::Event>>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ClientConnectionManger<E> {
    fn new(
        builder: ConnectionBuilder<E>,
        url: String,
        bind: BindMode,
        tx: UnboundedSender<ManagedEvent<E::Event>>,
    ) -> Self {
        Self {
            builder,
            url,
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

    type Error = crate::error::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        tracing::debug!(target: TARGET, "Connecting");

        let (client, mut events) = self.builder.clone().connect(&self.url).await?;

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
            .ok_or(crate::error::Error::ConnectionClosed)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        conn.is_closed()
    }
}
