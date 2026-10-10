// XXX: Only available with tokio, because tryhard only supports tokio.

use std::{
    collections::VecDeque,
    fmt::Debug,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};

use futures::{Stream, task::AtomicWaker};
use rusmpp::pdus::{BindReceiver, BindTransceiver, BindTransmitter};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{RwLock, RwLockReadGuard, watch},
};
use tryhard::backoff_strategies::{
    BackoffStrategy, ExponentialBackoff, FixedBackoff, LinearBackoff, NoBackoff,
};

use crate::{
    Client, ConnectionBuilder,
    error::Error,
    event_::EventChannel,
    runtime_::{Delay, Timeout, tokio::Tokio},
};

#[cfg(test)]
mod tests;

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

/// The lifecycle state of a managed connection.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedState {
    /// The transport is up. For a binding builder the bind handshake may still be running.
    Connected,
    /// The bind handshake completed: the client is bound and usable.
    Bound,
    /// The connection is gone. The next [`ManagedClient::get`] (or the automatic
    /// reconnection) opens a new generation.
    Disconnected,
}

/// A lifecycle snapshot: the connection's generation and its state.
///
/// Every successful connection is a new generation (`generation` increments), so a
/// reconnect is distinguishable from a stale status even when the state itself repeats
/// (`Bound -> Disconnected -> Bound`). Lifecycle is state: a slow observer only ever misses
/// intermediate values, and nothing that publishes it ever waits for one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagedStatus {
    generation: u64,
    state: ManagedState,
}

impl ManagedStatus {
    /// The generation this status belongs to: `1` for the first connection, incrementing on
    /// every reconnect. `0` before the first connection.
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// The lifecycle state.
    pub const fn state(&self) -> ManagedState {
        self.state
    }
}

/// Publishes a lifecycle transition atomically, and only while it is still the current
/// truth.
///
/// The comparison and the replacement happen in one step (under the watch's lock), so a
/// staler publication can never race a newer one, and two rules guard the ordering:
///
/// - a generation older than the published one is stale (a newer connection superseded
///   it), and
/// - a generation whose termination is already published is terminal: a delayed `Bound`
///   (or any later transition) must not resurrect it.
///
/// A no-op publication (the state is already what is being published) writes nothing.
fn publish_status(status: &watch::Sender<ManagedStatus>, generation: u64, state: ManagedState) {
    status.send_if_modified(|current| {
        if current.generation > generation {
            return false;
        }

        if current.generation == generation && current.state == ManagedState::Disconnected {
            return false;
        }

        let next = ManagedStatus { generation, state };

        if *current == next {
            return false;
        }

        *current = next;

        true
    });
}

/// A managed `SMPP` client that automatically handles reconnection and binding.
pub struct ManagedClient {
    inner: Arc<ManagedClientInner>,
    // Used to tell the reconnecting background task to stop when the client is dropped.
    _watch: watch::Receiver<()>,
    // The lifecycle watch (see [`ManagedClient::status`]). State, so observing it can not
    // block anything that publishes it.
    status: watch::Receiver<ManagedStatus>,
}

impl Clone for ManagedClient {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _watch: self._watch.clone(),
            status: self.status.clone(),
        }
    }
}

impl Debug for ManagedClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagedClient").finish()
    }
}

struct ManagedClientInner {
    creator: Box<dyn BoundClientCreator<Tokio>>,
    client: RwLock<Client<Tokio>>,
    /// Generations the public stream never drained (see [`GenerationQueue::push`]).
    dropped_generations: Arc<AtomicU64>,
    /// The client's handle on generation publication (see [`GenerationProducer`]): while a
    /// client handle is alive a reconnect can still produce a generation, so the public
    /// stream must stay open; the last handle (and the reconnect task that holds this
    /// inner behind it) closing this is what lets the stream end.
    _producer: GenerationProducer,
}

impl ManagedClientInner {
    fn new(
        creator: Box<dyn BoundClientCreator<Tokio>>,
        client: Client<Tokio>,
        dropped_generations: Arc<AtomicU64>,
        producer: GenerationProducer,
    ) -> Self {
        Self {
            creator,
            client: RwLock::new(client),
            dropped_generations,
            _producer: producer,
        }
    }

    async fn get(&self) -> Result<RwLockReadGuard<'_, Client<Tokio>>, Error> {
        {
            let client = self.client.read().await;

            if client.is_active() {
                return Ok(client);
            }
        }

        let mut client = self.client.write().await;

        // Another task may have reconnected while this one waited for the write lock:
        // replacing a live client would only discard the fresh connection. (The write lock
        // guards the client handle alone — nothing here ever waits for a consumer.)
        if client.is_active() {
            return Ok(client.downgrade());
        }

        *client = self.creator.connect().await?;

        Ok(client.downgrade())
    }
}

impl ManagedClient {
    fn new(
        inner: Arc<ManagedClientInner>,
        watch: watch::Receiver<()>,
        status: watch::Receiver<ManagedStatus>,
    ) -> Self {
        Self {
            inner,
            _watch: watch,
            status,
        }
    }

    /// Gets a connected and bound [`Client`].
    ///
    /// This method will block until a connected [`Client`] is available, and will automatically attempt to reconnect if the connection is lost.
    ///
    /// Reconnecting never waits for the event stream's consumer: lifecycle transitions are
    /// published as state (see [`ManagedClient::status`]), and a stream nobody drains only
    /// loses events where the connection's own bounded channel says so.
    pub async fn get(&self) -> Result<Client<Tokio>, Error> {
        self.inner.get().await.map(|client| client.clone())
    }

    /// The connection's lifecycle status (see [`ManagedStatus`]).
    ///
    /// A snapshot: each successful connection is a new generation, so a reconnect is
    /// visible even when the state itself repeats.
    pub fn status(&self) -> ManagedStatus {
        *self.status.borrow()
    }

    /// How many generations were discarded before the public event stream drained them.
    ///
    /// A generation waits to be drained by the public stream; more than
    /// [`GENERATIONS_CAP`](self) of them waiting means the consumer is not reading, and the
    /// stalest waiting one is discarded whole (and counted here) so a stopped consumer can
    /// never stall a reconnection. Monotonic for the life of the [`ManagedClient`].
    pub fn dropped_generations(&self) -> u64 {
        self.inner.dropped_generations.load(Ordering::Relaxed)
    }

    /// Gets a connected and bound [`Client`] with a timeout.
    pub async fn get_with_timeout(
        &self,
        timeout: Duration,
    ) -> Option<Result<Client<Tokio>, Error>> {
        Tokio::timeout(timeout, self.get()).await
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

/// Builder for creating an unbound managed connection.
#[derive(Debug)]
pub struct UnboundManagedConnectionBuilder<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E, Tokio>,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> UnboundManagedConnectionBuilder<E> {
    pub(crate) fn new(builder: ConnectionBuilder<E, Tokio>) -> Self {
        Self { builder }
    }

    /// Binds the [`ManagedClient`] as a transmitter.
    ///
    /// Every time the client reconnects, it will automatically bind as a transmitter using the provided [`BindTransmitter`].
    pub fn transmitter(self, bind: BindTransmitter) -> ManagedConnectionBuilder<E> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Transmitter(bind))
    }

    /// Binds the [`ManagedClient`] as a receiver.
    ///
    /// Every time the client reconnects, it will automatically bind as a receiver using the provided [`BindReceiver`].
    pub fn receiver(self, bind: BindReceiver) -> ManagedConnectionBuilder<E> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Receiver(bind))
    }

    /// Binds the [`ManagedClient`] as a transceiver.
    ///
    /// Every time the client reconnects, it will automatically bind as a transceiver using the provided [`BindTransceiver`].
    pub fn transceiver(self, bind: BindTransceiver) -> ManagedConnectionBuilder<E> {
        ManagedConnectionBuilder::new(self.builder, BindMode::Transceiver(bind))
    }

    /// Does not bind the [`ManagedClient`].
    ///
    /// Every time the client reconnects, it will not automatically bind.
    pub fn unbound(self) -> ManagedConnectionBuilder<E> {
        ManagedConnectionBuilder::new(self.builder, BindMode::None)
    }
}

/// Builder for creating a managed connection.
#[derive(Debug)]
pub struct ManagedConnectionBuilder<E: EventChannel + Clone + Send + Sync + 'static> {
    builder: ConnectionBuilder<E, Tokio>,
    bind: BindMode,
    auto_reconnect_interval: Option<Duration>,
    max_delay: Option<Duration>,
    back_off: BackOff,
    max_retries: u32,
}

impl<E: EventChannel + Clone + Send + Sync + 'static> ManagedConnectionBuilder<E> {
    fn new(builder: ConnectionBuilder<E, Tokio>, bind: BindMode) -> Self {
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

impl<E: EventChannel> ManagedConnectionBuilder<E>
where
    E: Clone + Send + Sync + 'static,
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
        Error,
    > {
        // The generations waiting to be drained by the public stream. The stream polls
        // their event streams directly — nothing copies events between the connection and
        // the application — so a no-wait completion's reserved credit is released on real
        // consumption, and a generation is only discarded when it is the stalest one
        // waiting (counted), never the fresh connection.
        let generations = Arc::new(GenerationQueue::new());

        // The lifecycle watch: publishing is synchronous and coalescing (state), so nothing
        // in the connection path ever waits for a consumer.
        let (status_tx, status_rx) = watch::channel(ManagedStatus {
            generation: 0,
            state: ManagedState::Disconnected,
        });

        let creator = BoundClientCreatorImpl::new(
            self.builder,
            connect,
            self.bind,
            self.max_delay,
            self.back_off,
            self.max_retries,
            status_tx,
            Arc::clone(&generations),
        );

        let client = creator.connect().await?;
        let client = Arc::new(ManagedClientInner::new(
            Box::new(creator),
            client,
            generations.dropped_generations(),
            generations.producer(),
        ));

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
                        _ = Tokio::delay(interval) => {
                            tracing::trace!(target: TARGET, "Triggering reconnection");

                            // The reconnect attempt is itself inside the select: the last
                            // client's drop must cancel a reconnect in flight (a stalled
                            // one can retry for minutes), not be observed only once get()
                            // returns. Cancelling is safe — the attempt is a plain
                            // connect future, and an aborted attempt leaves nothing
                            // registered.
                            tokio::select! {
                                _ = w_tx.closed() => {
                                    tracing::debug!(target: TARGET, "Stopping reconnect task mid-attempt");

                                    break;
                                }
                                result = client_c.get() => {
                                    if let Err(err) = result {
                                        tracing::error!(target: TARGET, ?err, "Failed to reconnect");
                                    }
                                }
                            }
                        }
                    }
                }

                tracing::trace!(target: TARGET, "Reconnect task stopped");
            });
        }

        let events = ManagedEventsStream {
            queue: generations,
            current: None,
            pending: VecDeque::new(),
        };

        Ok((ManagedClient::new(client, w_rx, status_rx), events))
    }

    /// Sets a function to be called when connecting.
    ///
    /// See [`ConnectionBuilder::connected`] for more details.
    pub async fn connect_fn<F, Fut, S>(
        self,
        f: F,
    ) -> Result<
        (
            ManagedClient,
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
            ManagedClient,
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

struct BoundClientCreatorImpl<E: EventChannel, R: Delay + Timeout> {
    builder: ConnectionBuilder<E, R>,
    connect: Connect,
    bind: BindMode,
    max_delay: Option<Duration>,
    back_off: BackOff,
    max_retries: u32,
    /// The lifecycle watch: every publish is a state overwrite, so it never waits.
    status: watch::Sender<ManagedStatus>,
    /// The generations waiting to be drained by the public stream.
    generations: Arc<GenerationQueue<E>>,
    /// The next generation number.
    generation: AtomicU64,
}

impl<E: EventChannel, R: Delay + Timeout> BoundClientCreatorImpl<E, R>
where
    E: Clone + Send + Sync + 'static,
    E::Event: Send + Sync + 'static,
    R: Clone + Send + Sync + 'static,
    <R as Delay>::Future: Send,
{
    #[allow(clippy::too_many_arguments)]
    fn new(
        builder: ConnectionBuilder<E, R>,
        connect: Connect,
        bind: BindMode,
        max_delay: Option<Duration>,
        back_off: BackOff,
        max_retries: u32,
        status: watch::Sender<ManagedStatus>,
        generations: Arc<GenerationQueue<E>>,
    ) -> Self {
        Self {
            builder,
            connect,
            bind,
            max_delay,
            back_off,
            max_retries,
            status,
            generations,
            generation: AtomicU64::new(0),
        }
    }
}

impl<E: EventChannel> BoundClientCreatorImpl<E, Tokio>
where
    E: Clone + Send + Sync + 'static,
    E::Event: Send + Sync + 'static,
{
    async fn connect_(&self) -> Result<Client<Tokio>, Error> {
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

        let (client, events) = fut.await?;

        let generation = self.generation.fetch_add(1, Ordering::Relaxed) + 1;

        // Publish the lifecycle as state: the send is synchronous and coalescing, so a
        // consumer that never reads its stream can not hold the connection up.
        publish_status(&self.status, generation, ManagedState::Connected);

        tracing::debug!(target: TARGET, "Connected");

        // A termination-only watcher, installed before the bind: `Disconnected` must reach
        // the lifecycle state from the connection's own end, never from the application
        // polling its stream (a stream nobody reads must not leave the status at `Bound`).
        // It holds a clone of the connection's watch *sender* — not a receiver, so the
        // "all receivers dropped" condition it awaits is untouched, and nothing that keeps
        // the connection alive (its actions sender) is retained.
        {
            let termination = client.termination_watch();
            let status = self.status.clone();

            Tokio::spawn(async move {
                termination.closed().await;

                publish_status(&status, generation, ManagedState::Disconnected);

                tracing::debug!(target: TARGET, generation, "Terminated: Disconnected published");
            });
        }

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
            publish_status(&self.status, generation, ManagedState::Bound);

            tracing::debug!(target: TARGET, "Bound");
        }

        // Hand the generation to the public stream. It polls this stream directly, so
        // nothing copies events between the connection and the application: a no-wait
        // completion's reserved credit is released on real consumption, and the
        // connection's own bounded channel (which warns and counts its drops) stays the
        // single place this generation's events can be lost.
        self.generations.push(Generation {
            sequence: generation,
            bound: self.bind.is_bind(),
            events: Box::pin(events),
        });

        Ok(client)
    }
}

trait BoundClientCreator<T>: Send + Sync + 'static {
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Client<T>, Error>> + Send + '_>>;
}

impl<E: EventChannel> BoundClientCreator<Tokio> for BoundClientCreatorImpl<E, Tokio>
where
    E: Clone + Send + Sync + 'static,
    E::Event: Send + Sync + 'static,
{
    fn connect(&self) -> Pin<Box<dyn Future<Output = Result<Client<Tokio>, Error>> + Send + '_>> {
        Box::pin(async move { self.connect_().await })
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

/// How many generations may wait to be drained by the public event stream.
///
/// A generation only waits while the consumer is behind; more than this many waiting means
/// the consumer is not reading at all, and the stalest waiting one is discarded (counted)
/// rather than retaining an unbounded backlog of dead connections.
const GENERATIONS_CAP: usize = 4;

/// One successful connection, waiting to be drained by the public event stream.
struct Generation<E: EventChannel> {
    /// The generation number (see [`ManagedStatus::generation`]).
    sequence: u64,
    /// Whether the bind handshake completed for this generation.
    bound: bool,
    /// The connection's event stream, polled directly by the public stream.
    events: Pin<Box<dyn Stream<Item = E::Event> + Send + 'static>>,
}

/// The publication state of a managed client's generation queue: explicit producer
/// ownership, plus what the public stream needs to know when production is over.
struct Publication {
    /// Live producers of generations (see [`GenerationProducer`]).
    producers: AtomicU64,
    /// Set by the last producer's drop: no generation can ever be pushed again.
    closed: AtomicBool,
    waker: AtomicWaker,
}

/// One handle on generation publication.
///
/// A generation can be produced while a client handle is alive (a live client can
/// reconnect) and by the in-flight connect a reconnect task may be running; while any
/// producer exists the public stream must stay open. Dropping the **last** producer closes
/// publication: the stream drains what remains — the current generation, the queued ones,
/// the pending lifecycle transitions — and then ends, instead of parking forever after the
/// last client is gone.
struct GenerationProducer {
    publication: Arc<Publication>,
}

impl GenerationProducer {
    fn new(publication: Arc<Publication>) -> Self {
        publication.producers.fetch_add(1, Ordering::Relaxed);

        Self { publication }
    }
}

impl Drop for GenerationProducer {
    fn drop(&mut self) {
        if self.publication.producers.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.publication.closed.store(true, Ordering::Release);
            self.publication.waker.wake();

            tracing::debug!(target: TARGET, "the last generation producer is gone: the public stream ends once drained");
        }
    }
}

/// The generations waiting to be drained by the public event stream, oldest first.
///
/// Publishing never waits for a consumer: a full queue discards the **stalest** waiting
/// generation — not the fresh connection — and counts the drop. The current connection's
/// own bounded channel remains the single place its events can be lost with the connection
/// still up.
struct GenerationQueue<E: EventChannel> {
    generations: Mutex<VecDeque<Generation<E>>>,
    publication: Arc<Publication>,
    dropped_generations: Arc<AtomicU64>,
}

impl<E: EventChannel> GenerationQueue<E> {
    fn new() -> Self {
        Self {
            generations: Mutex::new(VecDeque::new()),
            publication: Arc::new(Publication {
                producers: AtomicU64::new(0),
                closed: AtomicBool::new(false),
                waker: AtomicWaker::new(),
            }),
            dropped_generations: Arc::new(AtomicU64::new(0)),
        }
    }

    fn dropped_generations(&self) -> Arc<AtomicU64> {
        Arc::clone(&self.dropped_generations)
    }

    /// Takes one producer handle on generation publication (see [`GenerationProducer`]).
    fn producer(&self) -> GenerationProducer {
        GenerationProducer::new(Arc::clone(&self.publication))
    }

    /// Whether every producer is gone: no generation can ever be pushed again.
    fn is_closed(&self) -> bool {
        self.publication.closed.load(Ordering::Acquire)
    }

    /// Publishes a generation for the public stream. Never waits.
    fn push(&self, generation: Generation<E>) {
        {
            let mut generations = self.generations.lock().unwrap();

            while generations.len() >= GENERATIONS_CAP {
                let Some(discarded) = generations.pop_front() else {
                    break;
                };

                self.dropped_generations.fetch_add(1, Ordering::Relaxed);

                tracing::warn!(
                    target: TARGET,
                    generation = discarded.sequence,
                    "the public event stream is not draining its generations: discarding the stalest waiting one"
                );
            }

            generations.push_back(generation);
        }

        self.publication.waker.wake();
    }

    fn pop(&self) -> Option<Generation<E>> {
        self.generations.lock().unwrap().pop_front()
    }

    fn register_waker(&self, waker: &std::task::Waker) {
        self.publication.waker.register(waker);
    }
}

// The public event stream of a managed client.
//
// It polls the generations' event streams directly — no relay task copies events — so a
// reserved delivery's credit is released when the application consumes the event, exactly
// as on the connection's own stream, and a stopped consumer loses events only where the
// connection's bounded channel says so (warned and counted). Lifecycle transitions are
// surfaced as ManagedEvent::Connected, ManagedEvent::Bound and ManagedEvent::Disconnected
// around each generation; the lifecycle *status* is published by the connection's own
// termination watcher, not by this stream (see `publish_status`).
//
// No `#[pin]` field: the stream is `Unpin` unconditionally (an event channel's `Event`
// need not be), which is why the struct is projected rather than reached through
// `get_mut`. `pending` holds the lifecycle transitions still to be yielded, oldest first.
// (Field doc comments are impossible here: pin_project_lite rejects attributes on fields.)
pin_project_lite::pin_project! {
    struct ManagedEventsStream<E: EventChannel> {
        queue: Arc<GenerationQueue<E>>,
        current: Option<Generation<E>>,
        pending: VecDeque<ManagedEvent<E::Event>>,
    }
}

/// Starts a generation for the public stream: its lifecycle transitions first, then its
/// events.
fn start_generation<E: EventChannel>(
    generation: Generation<E>,
    current: &mut Option<Generation<E>>,
    pending: &mut VecDeque<ManagedEvent<E::Event>>,
) {
    pending.push_back(ManagedEvent::Connected);

    if generation.bound {
        pending.push_back(ManagedEvent::Bound);
    }

    *current = Some(generation);
}

impl<E: EventChannel> Stream for ManagedEventsStream<E> {
    type Item = ManagedEvent<E::Event>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        loop {
            if let Some(event) = this.pending.pop_front() {
                return Poll::Ready(Some(event));
            }

            if let Some(generation) = this.current.as_mut() {
                match generation.events.as_mut().poll_next(cx) {
                    Poll::Ready(Some(event)) => {
                        return Poll::Ready(Some(ManagedEvent::Event(event)));
                    }
                    Poll::Ready(None) => {
                        let sequence = generation.sequence;

                        *this.current = None;
                        this.pending.push_back(ManagedEvent::Disconnected);

                        // The status transition (`Disconnected`) is published by the
                        // generation's termination watcher, not here: the lifecycle state
                        // must not depend on this stream being polled.

                        tracing::warn!(target: TARGET, generation = sequence, "Disconnected");

                        continue;
                    }
                    // The current generation is not finished — a temporary unreadiness
                    // (a cooperative-budget yield, an idle connection) is not its end.
                    // Advancing to the queue here would replace it and silently drop its
                    // buffered events and its Disconnected: only `Ready(None)` ends a
                    // generation. The waker the poll just registered keeps this stream
                    // live; while the current generation parks, nothing a newer
                    // generation holds could be yielded anyway.
                    Poll::Pending => return Poll::Pending,
                }
            }

            match this.queue.pop() {
                Some(generation) => {
                    start_generation(generation, this.current, this.pending);

                    continue;
                }
                None => {
                    this.queue.register_waker(cx.waker());

                    // A push or a producer's close may have landed between the pop and the
                    // registration: re-check both before parking. The order matters — the
                    // waker is registered first, so a close that lands after this point
                    // wakes the stream instead of sleeping through it.
                    match this.queue.pop() {
                        Some(generation) => {
                            start_generation(generation, this.current, this.pending);

                            continue;
                        }
                        None => {
                            if this.queue.is_closed() {
                                // Every producer is gone and everything they published has
                                // been drained: no generation can ever come again, so the
                                // stream ends instead of parking forever.
                                return Poll::Ready(None);
                            }

                            // A client can still reconnect, so the stream stays open.
                            return Poll::Pending;
                        }
                    }
                }
            }
        }
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
