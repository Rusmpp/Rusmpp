use std::{
    collections::{BTreeMap, VecDeque},
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};

use crate::{
    Action, Client, Request, RequestId, Timer,
    builder_::NoSpawnConnectionBuilder,
    error::{Error, NotSentReason},
    event_::{EventChannel, Insight},
    request::{LateHandle, ObligatedRequest, Outcome, PendingEntry},
    runtime_::{Delay, Timeout},
};
use futures::{FutureExt, Sink, SinkExt, Stream, task::AtomicWaker};
use pin_project_lite::pin_project;
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    tokio_codec::{DecodeError, EncodeError},
};
use tokio::sync::{
    mpsc::{self, UnboundedSender, error::TrySendError},
    watch,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

const CONN: &str = "rusmppc::connection::smpp";
const TIMER: &str = "rusmppc::connection::smpp::timer";

/// Cap on unresolved written registrations: every request the connection has written whose
/// response it still owes, live and abandoned alike.
///
/// Each one keeps its sequence number reserved until its response claims it (or the
/// connection-generation ends): releasing one early would let a late reply be delivered to a
/// newer request that reused the number. The bound is therefore enforced by **admission**,
/// never by eviction — eviction would knowingly sacrifice identity correctness, retention
/// alone would have no bound — so at capacity a new application write is refused with a
/// `NotSent` verdict before anything is assigned, registered or written. Protocol controls
/// (obligated requests) are never refused, and reads continue: the responses in flight are
/// what frees the capacity again.
const UNRESOLVED_REQUESTS_CAP: usize = 1024;

const ACTIONS_POLL_LIMIT: u8 = 5;
const SINK_POLL_LIMIT: u8 = 5;
const STREAM_POLL_LIMIT: u8 = 5;
/// How many late replies one poll pass may surface (see the late-lane drain).
const LATE_POLL_LIMIT: u8 = 5;

#[derive(Debug)]
enum State {
    Active,
    /// The user sent a close request.
    Closing,
    Errored,
}

/// The late lane: replies whose caller is gone, waiting to be surfaced.
///
/// A private mutex-backed monitor, deliberately NOT an mpsc channel: enqueue and
/// close must share ONE linearization point. An mpsc `send` can be in flight when
/// the receiver closes — the send observes nothing about the closure and reports
/// success — and the message is then discarded by the receiver's destruction,
/// counted nowhere; and `try_recv` after `close()` only reports `Disconnected`
/// once drained on tokio versions that check closure before emptiness, while this
/// crate accepts any `tokio = "1"` (the previous sweep's `debug_assert!` panicked
/// on tokio 1.47/1.48 debug builds). Here, `enqueue` wins the lock and transfers
/// ownership to the draining side, or `close_and_take` wins it and the SENDER
/// counts its own rejection — there is no accepted-but-unpublished state outside
/// the lock, on any tokio.
///
/// LOCK DISCIPLINE, load-bearing: the lock is held for queue operations only.
/// No event callbacks (user code), no waker registration/waking/dropping, no
/// logging, no request-cell acquisition, no joins under it. Enqueue wakes AFTER
/// unlocking; a forward can therefore briefly block behind the connection's short
/// critical sections, but it cannot deadlock against them, and the caller path
/// (the request cell) releases its own lock before forwarding.
#[derive(Debug)]
pub(crate) struct LateLane {
    state: Mutex<LateLaneState>,
    /// Registered by the connection BEFORE each queue inspection (AtomicWaker
    /// contract), woken after each enqueue, cleared at teardown.
    waker: AtomicWaker,
    #[cfg(test)]
    test_hooks: LateLaneTestHooks,
}

#[derive(Debug, Default)]
struct LateLaneState {
    closed: bool,
    queue: VecDeque<Command>,
}

#[cfg(test)]
#[derive(Default)]
struct LateLaneTestHooks {
    /// Runs IMMEDIATELY BEFORE the lock acquisition in `enqueue`.
    before_lock: Option<Box<dyn Fn() + Send + Sync>>,
    /// Runs while the guard is held, AFTER `closed` was observed false and
    /// BEFORE the push — the acceptance/publication boundary pinned
    /// deterministically, where tokio's own increment-before-push split lives.
    in_window: Option<Box<dyn Fn() + Send + Sync>>,
}

#[cfg(test)]
impl std::fmt::Debug for LateLaneTestHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LateLaneTestHooks")
            .field("before_lock", &self.before_lock.is_some())
            .field("in_window", &self.in_window.is_some())
            .finish()
    }
}

impl LateLane {
    pub(crate) fn new() -> Self {
        Self {
            state: Mutex::new(LateLaneState::default()),
            waker: AtomicWaker::new(),
            #[cfg(test)]
            test_hooks: LateLaneTestHooks::default(),
        }
    }

    /// The waker the connection registers before inspecting the queue.
    pub(crate) fn waker(&self) -> &AtomicWaker {
        &self.waker
    }

    /// Enqueue unless the lane is closed. Returns `false` for a rejection so
    /// the SENDER (the `LateHandle`) counts it — this side never counts.
    pub(crate) fn enqueue(&self, command: Command) -> bool {
        #[cfg(test)]
        if let Some(hook) = &self.test_hooks.before_lock {
            hook();
        }

        {
            let mut state = self.state.lock().unwrap();

            if state.closed {
                return false;
            }

            #[cfg(test)]
            if let Some(hook) = &self.test_hooks.in_window {
                hook();
            }

            state.queue.push_back(command);
        }

        // OUTSIDE the lock: waking runs a user-supplied waker.
        self.waker.wake();

        true
    }

    /// Take at most `limit` commands; reports whether more remain. The
    /// connection self-wakes when more remain, so a full lane can never strand
    /// behind the cooperative budget.
    fn take_up_to(&self, limit: usize) -> (Vec<Command>, bool) {
        let mut state = self.state.lock().unwrap();

        let mut batch = Vec::with_capacity(limit.min(state.queue.len()));

        for _ in 0..limit {
            match state.queue.pop_front() {
                Some(command) => batch.push(command),
                None => break,
            }
        }

        let more = !state.queue.is_empty();

        (batch, more)
    }

    /// Teardown. In ONE critical section: close the lane and take everything it
    /// holds; a concurrent forward therefore either lands in the returned set
    /// or observes `closed` and counts its own rejection — nothing else is
    /// possible, which is what makes "surfaced or counted" exhaustive. The
    /// stored waker is cleared OUTSIDE the lock, because dropping a waker runs
    /// user code too.
    fn close_and_take(&self) -> Vec<Command> {
        let taken = {
            let mut state = self.state.lock().unwrap();

            state.closed = true;

            std::mem::take(&mut state.queue)
        };

        self.waker.take();

        taken.into()
    }
}

#[cfg(test)]
impl LateLane {
    /// Installs the deterministic gates the lane probes need; test-only.
    pub(crate) fn set_test_hooks(
        &mut self,
        before_lock: Option<Box<dyn Fn() + Send + Sync>>,
        in_window: Option<Box<dyn Fn() + Send + Sync>>,
    ) {
        self.test_hooks = LateLaneTestHooks {
            before_lock,
            in_window,
        };
    }
}

/// The connection's event channel plus the late lane it must flush at teardown.
///
/// A late reply that was accepted into the lane is owed to the application however the
/// connection ends, so the sink's drop closes the lane and surfaces whatever it holds
/// through the event channel — or counts it when the channel can no longer take it. (The
/// lane is also drained a bounded batch per poll pass — `LATE_POLL_LIMIT` with a self-wake
/// while work remains; the sweep covers what lands between the last drain and the drop.)
/// `Deref` to the event channel keeps every call site reading as the plain channel.
struct EventSink<E: EventChannel> {
    events: E,
    /// The late lane, drained in `poll` and swept here: a late reply is an
    /// incoming command that no registration claimed.
    late: Arc<LateLane>,
    /// How many late responses — replies whose caller is gone — could not be delivered to
    /// the application: the forward to this connection's late lane found it closed, or the
    /// event channel refused the surfaced command. Shared with the client (read through
    /// `Client::late_responses_dropped`): the failure can happen at either end of the lane,
    /// and the loss must be visible even though the event stream is, by definition, not
    /// where it shows up.
    late_responses_dropped: Arc<AtomicU64>,
}

impl<E: EventChannel> std::ops::Deref for EventSink<E> {
    type Target = E;

    fn deref(&self) -> &E {
        &self.events
    }
}

impl<E: EventChannel> std::ops::DerefMut for EventSink<E> {
    fn deref_mut(&mut self) -> &mut E {
        &mut self.events
    }
}

impl<E: EventChannel> Drop for EventSink<E> {
    fn drop(&mut self) {
        // Teardown: close the lane and take EVERYTHING it holds in one critical
        // section, then surface-or-count each command. Closing is structural:
        // a forward that loses the race observes `closed` and counts its own
        // rejection at the sender, so nothing can be accepted after this take
        // and then discarded — the two hazards of the old mpsc dance (a send in
        // flight during close; `try_recv`'s version-dependent terminator) do
        // not exist under one lock. Delivery happens OUTSIDE the lock: the
        // event channel is user-implemented code.
        for command in self.late.close_and_take() {
            if self.events.send_incoming(command).is_err() {
                self.late_responses_dropped.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

// Make sure to drop the Connection after completion to prevent clients from queueing more actions.
// This way if the Connection was closed and the Connection is not in an active state but not dropped,
// clients will still be able to send actions and will not get an immediate error that the channel is closed.
// Actions will not be queued and the client would wait forever until the Connection is dropped.
// We rely on this mechanism to work, to report correct and predictable errors.
pin_project! {
    pub struct Connection<F, E: EventChannel, D: Delay> {
        state: State,
        sequence_number: u32,
        requests: VecDeque<Request>,
        // This is a request that has been written to the sink using start_send, but not yet flushed.
        pending_request: Option<Request>,
        // The sequence-occupancy table: one entry per written request the connection still
        // owes an outcome. The keys are the reserved sequence numbers: a request is written
        // with a number only if it is not in here, and the number stays reserved until the
        // response claims the entry or the connection ends. An entry whose request cell is
        // abandoned stays unresolved: the caller walked away after the write, and the
        // number must still not be reused while its late reply can arrive (see
        // UNRESOLVED_REQUESTS_CAP for the admission bound).
        pending: BTreeMap<u32, PendingEntry>,
        // The late lane, installed on each request's state cell at the write gate, so a
        // response a gone caller leaves behind can still be surfaced as an incoming event.
        // A forward can briefly BLOCK behind the lane's short critical sections (it takes
        // the lane's mutex), but it never retains the connection and never runs user code
        // under that lock; a lane whose teardown has run (closed) makes the forward fail,
        // and that failure is counted at the sender (see `LateHandle`).
        late_lane: Arc<LateLane>,
        enquire_link_interval: Option<Duration>,
        last_enquire_link_sequence_number: Option<u32>,
        enquire_link_response_timeout: Duration,
        auto_enquire_link_response: bool,
        // The event channel plus the late lane it must flush at teardown (see
        // [`EventSink`]): the `Deref` keeps every use reading as the plain channel.
        events: EventSink<E>,
        // Used to let the client wait for the connection to be closed
        _watch: watch::Receiver<()>,
        #[pin]
        enquire_link_timer: Timer<D>,
        #[pin]
        enquire_link_response_timer: Timer<D>,
        #[pin]
        framed: F,
        #[pin]
        actions: UnboundedReceiverStream<Action>,
    }
}

impl<E: EventChannel, D: Delay> Connection<(), E, D> {
    // The wide tuple is internal plumbing: every element is consumed immediately by the
    // one caller (`raw`), which wraps each channel into its stream or client handle.
    #[allow(clippy::type_complexity)]
    pub fn new(
        enquire_link_interval: Option<Duration>,
        enquire_link_response_timeout: Duration,
        auto_enquire_link_response: bool,
    ) -> (
        Self,
        watch::Sender<()>,
        UnboundedSender<Action>,
        UnboundedReceiverStream<E::Event>,
        Arc<AtomicU64>,
    ) {
        let (events_tx, events_rx) = mpsc::unbounded_channel::<E::Event>();
        let events = E::new(events_tx);

        let (actions_tx, actions_rx) = mpsc::unbounded_channel::<Action>();
        let late_lane = Arc::new(LateLane::new());
        let (watch_tx, watch_rx) = watch::channel(());

        let late_responses_dropped = Arc::new(AtomicU64::new(0));

        (
            Self {
                state: State::Active,
                sequence_number: 2,
                requests: VecDeque::new(),
                pending_request: None,
                pending: BTreeMap::new(),
                late_lane: Arc::clone(&late_lane),
                enquire_link_interval,
                last_enquire_link_sequence_number: None,
                enquire_link_response_timeout,
                auto_enquire_link_response,
                enquire_link_timer: enquire_link_interval
                    .map(|duration| Timer::active(duration))
                    .unwrap_or(Timer::inactive()),
                enquire_link_response_timer: Timer::inactive(),
                _watch: watch_rx,
                events: EventSink {
                    events,
                    late: Arc::clone(&late_lane),
                    late_responses_dropped: Arc::clone(&late_responses_dropped),
                },
                framed: (),
                actions: UnboundedReceiverStream::new(actions_rx),
            },
            watch_tx,
            actions_tx,
            UnboundedReceiverStream::new(events_rx),
            late_responses_dropped,
        )
    }

    pub fn with_framed<F>(self, framed: F) -> Connection<F, E, D> {
        Connection {
            state: self.state,
            sequence_number: self.sequence_number,
            requests: self.requests,
            pending_request: self.pending_request,
            pending: self.pending,
            late_lane: self.late_lane,
            enquire_link_interval: self.enquire_link_interval,
            last_enquire_link_sequence_number: self.last_enquire_link_sequence_number,
            enquire_link_response_timeout: self.enquire_link_response_timeout,
            auto_enquire_link_response: self.auto_enquire_link_response,
            events: self.events,
            _watch: self._watch,
            enquire_link_timer: self.enquire_link_timer,
            enquire_link_response_timer: self.enquire_link_response_timer,
            framed,
            actions: self.actions,
        }
    }
}

impl<F, E, D: Delay> Connection<F, E, D>
where
    F: Stream<Item = Result<Command, DecodeError>> + for<'a> Sink<&'a Command, Error = EncodeError>,
    E: EventChannel,
{
    /// Fails a request that was handed to the sink (`start_send` accepted it), but whose
    /// write did not complete.
    ///
    /// The bytes are no longer retractable: the failure is a maybe-sent verdict, never a
    /// "not sent". The registration is withdrawn first — and only when it is still this
    /// request's: if the response already claimed it, the response won and nothing is sent
    /// (a response always wins over a later failure).
    fn fail_written_request(
        self: Pin<&mut Self>,
        sequence_number: u32,
        id: RequestId,
        error: Error,
    ) {
        let this = self.project();

        let Some(entry) = this.pending.remove(&sequence_number) else {
            // The response already claimed the registration: the caller has its outcome.
            return;
        };

        if entry.id != id {
            // The number now belongs to a newer request: this request's own registration was
            // claimed by its response, and the newer request's entry must stay.
            this.pending.insert(sequence_number, entry);

            return;
        }

        let _ = entry.outcome.try_send(Outcome::Failed(error));
    }

    fn requests_push_back(self: Pin<&mut Self>, request: Request) {
        self.project().requests.push_back(request);
    }

    fn requests_push_front(self: Pin<&mut Self>, request: Request) {
        self.project().requests.push_front(request);
    }

    fn requests_pop_front(self: Pin<&mut Self>) -> Option<Request> {
        self.project().requests.pop_front()
    }

    fn set_pending_request(self: Pin<&mut Self>, request: Request) {
        *self.project().pending_request = Some(request);
    }

    fn take_pending_request(self: Pin<&mut Self>) -> Option<Request> {
        self.project().pending_request.take()
    }

    fn set_state(self: Pin<&mut Self>, state: State) {
        *self.project().state = state;
    }

    fn deactivate_enquire_link_timer(self: Pin<&mut Self>) {
        self.project().enquire_link_timer.deactivate();

        tracing::trace!(target: TIMER, "Deactivated enquire_link_timer");
    }

    fn activate_enquire_link_timer(self: Pin<&mut Self>) {
        if let Some(delay) = self.as_ref().enquire_link_interval {
            self.project().enquire_link_timer.activate(delay);

            tracing::trace!(target: TIMER, ?delay, "Activated enquire_link_timer");
        }
    }

    fn set_last_enquire_link_sequence_number(self: Pin<&mut Self>, sequence_number: u32) {
        *self.project().last_enquire_link_sequence_number = Some(sequence_number);
    }

    fn unset_last_enquire_link_sequence_number(self: Pin<&mut Self>) {
        *self.project().last_enquire_link_sequence_number = None;
    }

    fn deactivate_enquire_link_response_timer(self: Pin<&mut Self>) {
        self.project().enquire_link_response_timer.deactivate();

        tracing::trace!(target: TIMER, "Deactivated enquire_link_response_timer");
    }

    fn activate_enquire_link_response_timer(self: Pin<&mut Self>) {
        let delay = self.as_ref().enquire_link_response_timeout;

        self.project().enquire_link_response_timer.activate(delay);

        tracing::trace!(target: TIMER, ?delay, "Activated enquire_link_response_timer");
    }

    /// [`Self::sequence_number`] is incremented by 2 after each call.
    ///
    /// The clients also hold an atomic sequence number, which is incremented by 2 for each request, starting from 1.
    ///
    /// This is done to ensure that commands sent by the connection [`EnquireLink`](Pdu::EnquireLink) are differentiated from the commands sent by the client,
    /// without the use of atomic operations in the connection.
    fn sequence_number_fetch_and_increment(self: Pin<&mut Self>) -> u32 {
        let sequence_number = self.sequence_number;

        // Even numbers only (the client owns the odd ones), inside the SMPP range: the
        // count wraps rather than leaving the range.
        *self.project().sequence_number =
            if sequence_number >= crate::client::MAX_SEQUENCE_NUMBER - 1 {
                2
            } else {
                sequence_number + 2
            };

        sequence_number
    }
}

impl<F, E, D: Delay> Future for Connection<F, E, D>
where
    F: Stream<Item = Result<Command, DecodeError>> + for<'a> Sink<&'a Command, Error = EncodeError>,
    E: EventChannel,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !matches!(self.state, State::Active | State::Closing) {
            return Poll::Ready(());
        }

        let mut stream_polls: u8 = 0;

        'main: loop {
            tracing::trace!(target: CONN, "Entering main poll loop");

            // The late lane: responses whose caller is gone, forwarded by the request cells
            // themselves (a drop, or a commit that raced an abandonment). They surface as
            // incoming events — a reply no registration claimed — and a delivery the event
            // channel can not take is counted, never silent. The waker is registered BEFORE
            // the queue is inspected (AtomicWaker's contract: a wake between registration
            // and inspection re-polls; a wake before registration is still observed because
            // the inspection below sees whatever was enqueued). A bounded batch per pass,
            // with a self-wake when more remain: draining without a quota spends the whole
            // cooperative budget here and starves the socket read and the queued writes,
            // while the remainder is delivered by the very next pass — and whatever is
            // still in the lane when the connection ends is swept by the sink's drop.
            {
                let this = self.as_mut().project();

                this.events.late.waker().register(cx.waker());

                let (batch, more) = this.events.late.take_up_to(usize::from(LATE_POLL_LIMIT));

                // Delivery is user code: it runs OUTSIDE the lane lock.
                for command in batch {
                    if this.events.send_incoming(command).is_err() {
                        this.events
                            .late_responses_dropped
                            .fetch_add(1, Ordering::Relaxed);
                    }
                }

                if more {
                    // More late responses are queued: schedule another poll before
                    // leaving (see the actions quota below).
                    cx.waker().wake_by_ref();
                }
            }

            if matches!(self.state, State::Active) {
                match self.as_mut().project().enquire_link_response_timer.poll(cx) {
                    Poll::Ready(()) => {
                        tracing::error!(target: TIMER, "EnquireLinkResp timeout");

                        self.as_mut().set_state(State::Errored);

                        let timeout = self.enquire_link_response_timeout;

                        let _ = self
                            .as_mut()
                            .events
                            .send_error(Error::EnquireLinkTimeout { timeout });

                        return Poll::Ready(());
                    }
                    Poll::Pending => {}
                }

                match self.as_mut().project().enquire_link_timer.poll(cx) {
                    Poll::Ready(()) => {
                        let sequence_number = self.as_mut().sequence_number_fetch_and_increment();

                        tracing::trace!(target: TIMER, sequence_number, "EnquireLink");

                        let command = Command::builder()
                            .status(CommandStatus::EsmeRok)
                            .sequence_number(sequence_number)
                            .pdu(Pdu::EnquireLink);

                        let request = ObligatedRequest::new(command);

                        self.as_mut()
                            .requests_push_front(Request::Obligated(request));

                        self.as_mut()
                            .set_last_enquire_link_sequence_number(sequence_number);
                        self.as_mut().deactivate_enquire_link_timer();
                        self.as_mut().activate_enquire_link_response_timer();

                        // Poll the enquire_link_response_timer again to register the waker
                        let _ = self.as_mut().project().enquire_link_response_timer.poll(cx);
                    }
                    Poll::Pending => {}
                }
            }

            if matches!(self.state, State::Active | State::Closing) {
                let mut i: u8 = 0;

                'actions: loop {
                    i += 1;

                    tracing::trace!(target: CONN, %i, "Entering actions poll loop");

                    if i > ACTIONS_POLL_LIMIT {
                        tracing::trace!(target: CONN, %i, "Exiting actions poll loop");

                        // More actions may be queued: schedule another poll before leaving.
                        // Without this wakeup the connection can end a poll with actions
                        // still queued and nothing left to wake it (an idle server sends
                        // nothing), stranding them until unrelated traffic arrives.
                        cx.waker().wake_by_ref();

                        break 'actions;
                    }

                    match self.as_mut().project().actions.poll_next(cx) {
                        Poll::Ready(Some(action)) => match action {
                            Action::Ping => {
                                // If we get here,
                                // this means that the connection is still active (did not close the actions channel) and can receive actions from the client.
                                // The client relies on the Action::Ping to be sent successfully to the connection, to determine if the connection is still active,
                                // using the `Client::is_active` method.
                            }
                            Action::PendingResponses(pending_responses) => {
                                let pending =
                                    self.as_mut().project().pending.keys().copied().collect();

                                let _ = pending_responses.ack.send(Ok(pending));
                            }
                            Action::Request(request) => {
                                tracing::debug!(target: CONN,
                                    sequence_number=request.command().sequence_number(),
                                    status=?request.command().status(),
                                    id=?request.command().id(),
                                    "Received request"
                                );

                                self.as_mut().requests_push_back(request);
                            }
                            Action::Cancel(id) => {
                                tracing::debug!(target: CONN, id, "Received cancel");

                                // A cleanup hint, never the authority: the request's cell
                                // already decided what the cancellation meant. Drop the
                                // queued request if it is still here — the write gate would
                                // refuse it anyway — while a written request stays
                                // unresolved (its number must not be reused while a late
                                // reply can still arrive).
                                self.as_mut().project().requests.retain(|request| {
                                    request.id() != Some(id) || !request.is_abandoned()
                                });
                            }
                            Action::Close(request) => {
                                tracing::debug!(target: CONN, "Received close");

                                self.as_mut().set_state(State::Closing);

                                self.as_mut().project().actions.close();

                                let _ = request.ack.send(());

                                continue 'main;
                            }
                        },
                        Poll::Ready(None) => {
                            if matches!(self.state, State::Closing) {
                                // We closed the channel to prevent more actions

                                break 'actions;
                            }

                            tracing::trace!(target: CONN, "Client dropped");

                            self.as_mut().set_state(State::Errored);

                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            tracing::trace!(target: CONN, "No pending actions");

                            break 'actions;
                        }
                    }
                }

                let mut i: u8 = 0;

                'sink: loop {
                    i += 1;

                    tracing::trace!(target: CONN, %i, "Entering sink poll loop");

                    if i > SINK_POLL_LIMIT {
                        tracing::trace!(target: CONN, %i, "Exiting sink poll loop");

                        // More writes may be queued: schedule another poll before leaving
                        // (see the actions quota above).
                        cx.waker().wake_by_ref();

                        break 'sink;
                    }

                    match self.as_mut().take_pending_request() {
                        Some(request) => {
                            let sequence_number = request.command().sequence_number();
                            let status = request.command().status();
                            let id = request.command().id();

                            tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Sending command");

                            match Sink::<&Command>::poll_flush(self.as_mut().project().framed, cx) {
                                Poll::Ready(Ok(_)) => {
                                    // The write acknowledgement: from here the caller knows
                                    // the request is with the transport. Not terminal — the
                                    // response is still owed.
                                    tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Sent command");

                                    match request {
                                        Request::Registered(request) => request.send_written(),
                                        Request::Unregistered(request) => request.send_written(),
                                        Request::Obligated(_) => {
                                            // No acknowledgement for obligated requests.
                                            match id {
                                                CommandId::EnquireLink => {
                                                    let _ = self.as_mut().events.send_insight(
                                                        Insight::SentEnquireLink(sequence_number),
                                                    );
                                                }
                                                CommandId::EnquireLinkResp => {
                                                    let _ = self.as_mut().events.send_insight(
                                                        Insight::SentEnquireLinkResp(
                                                            sequence_number,
                                                        ),
                                                    );
                                                }
                                                _ => {}
                                            }
                                        }
                                    }

                                    continue 'sink;
                                }
                                Poll::Ready(Err(err)) => {
                                    // The request was already handed to the sink: the bytes
                                    // are no longer retractable, so this is a maybe-sent
                                    // failure. A response that arrived first wins (it claimed
                                    // the registration, and `fail_written_request` leaves it
                                    // alone).
                                    tracing::error!(target: CONN, ?err);

                                    self.as_mut().set_state(State::Errored);

                                    if let Some(id) = request.id() {
                                        self.as_mut().fail_written_request(
                                            sequence_number,
                                            id,
                                            Error::from(err),
                                        );
                                    }

                                    return Poll::Ready(());
                                }
                                Poll::Pending => {
                                    self.as_mut().set_pending_request(request);

                                    tracing::trace!(target: CONN, "Sink poll flush pending");

                                    break 'sink;
                                }
                            }
                        }
                        None => {
                            tracing::trace!(target: CONN, "No pending request");
                        }
                    }

                    match self.as_mut().requests_pop_front() {
                        Some(mut request) => {
                            let sequence_number = request.command().sequence_number();

                            match Sink::<&Command>::poll_ready(self.as_mut().project().framed, cx) {
                                Poll::Ready(Ok(())) => {
                                    // Admission, before the gate: every unresolved written
                                    // registration holds its sequence number, so the table
                                    // is bounded by refusing new application writes — never
                                    // by evicting a reserved number (see
                                    // UNRESOLVED_REQUESTS_CAP). Protocol controls are
                                    // exempt: the conversation must go on, and reads
                                    // continue — they are what frees the capacity.
                                    if request.registers()
                                        && self.as_mut().project().pending.len()
                                            >= UNRESOLVED_REQUESTS_CAP
                                    {
                                        let reserved = self.as_mut().project().pending.len();

                                        tracing::warn!(target: CONN, reserved, cap = UNRESOLVED_REQUESTS_CAP, "Refusing a request: the connection's sequence-number reservations are at capacity");

                                        request.send_failed(Error::not_sent(
                                            NotSentReason::Capacity {
                                                reserved,
                                                cap: UNRESOLVED_REQUESTS_CAP,
                                            },
                                        ));

                                        continue 'sink;
                                    }

                                    // THE WRITE GATE. Check, assign the sequence number,
                                    // register and `start_send` in one synchronous step, and
                                    // only after the sink accepted the write: the request
                                    // becomes observable to the peer through `start_send`
                                    // alone, so a caller that abandoned it — cancelled, or
                                    // timed out while it was still queued — wins the race
                                    // exactly here, and nothing is registered or written for
                                    // it.
                                    let this = self.as_mut().project();

                                    let Some(claimed) = request.claim(this.pending) else {
                                        tracing::debug!(target: CONN, sequence_number, "Request abandoned before it was written; dropping it");

                                        continue 'sink;
                                    };

                                    // The request's cell gets its late lane before
                                    // `start_send`, the one point from which a response
                                    // becomes possible: a reply whose caller is gone by the
                                    // time it arrives — abandoned before, or racing, the
                                    // commit — is routed through it and surfaced as an
                                    // incoming event, even if the connection task itself is
                                    // gone by then (the forward is counted, not lost).
                                    // (Obligated requests have no cell: nothing awaits
                                    // them.)
                                    if let Request::Registered(request) = &request {
                                        request.cell.install_late(LateHandle::new(
                                            Arc::clone(this.late_lane),
                                            Arc::clone(&this.events.late_responses_dropped),
                                        ));
                                    }

                                    // The gate may have renumbered the request (its proposed
                                    // number was still reserved): read the written values
                                    // back after it.
                                    let sequence_number = request.command().sequence_number();
                                    let status = request.command().status();
                                    let id = request.command().id();

                                    tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Writing command");

                                    debug_assert_eq!(claimed, sequence_number);

                                    if let Err(err) =
                                        self.as_mut().project().framed.start_send(request.command())
                                    {
                                        // The transport refused the write before accepting
                                        // the bytes: the registration is withdrawn and the
                                        // caller is failed. (The codec encodes infallibly;
                                        // a generic sink refusing means it is broken.)
                                        tracing::error!(target: CONN, sequence_number, ?status, ?id, ?err);

                                        self.as_mut().set_state(State::Errored);

                                        if let Some(id) = request.id() {
                                            self.as_mut().fail_written_request(
                                                sequence_number,
                                                id,
                                                Error::from(err),
                                            );
                                        }

                                        return Poll::Ready(());
                                    }

                                    // `start_send` accepted the bytes: the request is written,
                                    // and its sequence number stays reserved until the
                                    // response claims the registration or the connection
                                    // ends.
                                    self.as_mut().set_pending_request(request);

                                    continue 'sink;
                                }
                                Poll::Ready(Err(err)) => {
                                    // The write could not begin: nothing was registered (the
                                    // gate runs only after a ready sink) and nothing was
                                    // written, so no response can exist for this request —
                                    // and the request is definitely not sent.
                                    tracing::error!(target: CONN, ?err);

                                    self.as_mut().set_state(State::Errored);

                                    if request.id().is_some() {
                                        request.send_failed(Error::not_sent(NotSentReason::Write(
                                            err,
                                        )));
                                    }

                                    return Poll::Ready(());
                                }
                                Poll::Pending => {
                                    // Back on the queue untouched: nothing was claimed,
                                    // registered or written, so a cancellation that arrives
                                    // while the sink is not ready still wins.
                                    self.as_mut().requests_push_front(request);

                                    tracing::trace!(target: CONN, "Sink poll ready pending");

                                    break 'sink;
                                }
                            }
                        }
                        None => {
                            tracing::trace!(target: CONN, "No requests in queue");

                            if matches!(self.state, State::Closing) {
                                tracing::debug!(target: CONN, "Closed");

                                // We set the state to `Errored` here to stop further processing in the next poll.
                                // This isn’t really an error — we could just as well call it `Closed`.
                                // `Errored` simply indicates that we are neither `Active` nor `Closing`.
                                self.as_mut().set_state(State::Errored);

                                return Poll::Ready(());
                            }

                            break 'sink;
                        }
                    }
                }
            }

            if matches!(self.state, State::Active) {
                'stream: loop {
                    stream_polls += 1;

                    tracing::trace!(target: CONN, i=%stream_polls, "Entering stream poll loop");

                    if stream_polls > STREAM_POLL_LIMIT {
                        tracing::trace!(target: CONN, i=%stream_polls, "Exiting stream poll loop");

                        tracing::debug!(target: CONN, "Pending");

                        cx.waker().wake_by_ref();

                        return Poll::Pending;
                    }

                    match self.as_mut().project().framed.poll_next(cx) {
                        Poll::Ready(Some(Ok(command))) => {
                            let sequence_number = command.sequence_number();
                            let status = command.status();
                            let id = command.id();

                            tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Received command");

                            // Auto respond to enquire link requests from the server only if auto_enquire_link_response is enabled.
                            if let CommandId::EnquireLink = command.id()
                                && self.auto_enquire_link_response
                            {
                                let response = Command::builder()
                                    .status(CommandStatus::EsmeRok)
                                    .sequence_number(command.sequence_number())
                                    .pdu(Pdu::EnquireLinkResp);

                                let request = ObligatedRequest::new(response);

                                self.as_mut()
                                    .requests_push_front(Request::Obligated(request));

                                let _ = self
                                    .as_mut()
                                    .events
                                    .send_insight(Insight::ReceivedEnquireLink(sequence_number));

                                continue 'main;
                            }

                            // Enquire link responses not matching the last sent enquire link are ignored and must be passed to the client. (The client sent an enquire link manually)
                            if let CommandId::EnquireLinkResp = command.id() {
                                if let Some(last_sequence_number) =
                                    self.last_enquire_link_sequence_number
                                {
                                    if let CommandStatus::EsmeRok = command.status() {
                                        if last_sequence_number == sequence_number {
                                            self.as_mut().unset_last_enquire_link_sequence_number();
                                            self.as_mut().deactivate_enquire_link_response_timer();
                                            self.as_mut().activate_enquire_link_timer();

                                            // Poll the enquire_link_timer again to register the waker
                                            let _ =
                                                self.as_mut().project().enquire_link_timer.poll(cx);

                                            let _ = self.as_mut().events.send_insight(
                                                Insight::ReceivedEnquireLinkResp(sequence_number),
                                            );

                                            continue 'stream;
                                        }
                                    }
                                }
                            }

                            if id.is_response() {
                                match self.as_mut().project().pending.remove(&sequence_number) {
                                    Some(entry) => {
                                        tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Found response");

                                        // The response always wins: it claimed the entry, so
                                        // no failure path can end this request any more, and
                                        // the sequence number is free again. The cell is
                                        // the single arbiter: the response is committed to
                                        // it first — it stores the payload for the caller,
                                        // or hands it back when the caller gave up before
                                        // the commit reached the cell — and the channel is
                                        // only how the caller learns there is something to
                                        // take.
                                        match entry.cell.commit_response(command) {
                                            Ok(()) => {
                                                match entry.outcome.try_send(Outcome::ResponseReady)
                                                {
                                                    Ok(()) => {
                                                        // Sent, do nothing
                                                    }
                                                    Err(TrySendError::Closed(_)) => {
                                                        // The caller's future is gone: its
                                                        // drop took the stored response
                                                        // and routed it late (the cell
                                                        // decided that atomically). If the
                                                        // drop has not run yet, it will
                                                        // find the response where the
                                                        // commit left it.
                                                        tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Client not waiting; the response is routed late by the caller's drop");
                                                    }
                                                    Err(TrySendError::Full(_)) => {
                                                        // Unreachable while the channel's
                                                        // capacity matches the protocol
                                                        // (one write acknowledgement plus
                                                        // one terminal): never silent.
                                                        tracing::error!(target: CONN, sequence_number, ?status, ?id, "Request outcome channel full; dropping the notification");
                                                    }
                                                }
                                            }
                                            Err(command) => {
                                                // The caller abandoned the request before
                                                // the commit reached the cell: the reply is
                                                // a late reply, surfaced as an incoming
                                                // event. A surfacing the event channel can
                                                // not take is a loss like any other late
                                                // delivery's: counted, never silent.
                                                tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Client not waiting");

                                                if self
                                                    .as_mut()
                                                    .events
                                                    .send_incoming(command)
                                                    .is_err()
                                                {
                                                    self.as_mut()
                                                        .project()
                                                        .events
                                                        .late_responses_dropped
                                                        .fetch_add(1, Ordering::Relaxed);
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        tracing::trace!(target: CONN, sequence_number, ?status, ?id, "No response found");

                                        // The client might have cancelled the request or it timed out.
                                        // In this case we just send the command as an incoming event.
                                        let _ = self.as_mut().events.send_incoming(command);
                                    }
                                }

                                continue 'stream;
                            }

                            // Command is an operation from the server.
                            let _ = self.as_mut().events.send_incoming(command);
                        }
                        Poll::Ready(Some(Err(err))) => {
                            tracing::error!(target: CONN, ?err);

                            self.as_mut().set_state(State::Errored);

                            let _ = self.as_mut().events.send_error(Error::from(err));

                            return Poll::Ready(());
                        }
                        Poll::Ready(None) => {
                            tracing::debug!(target: CONN, "Connection closed by the server");

                            self.as_mut().set_state(State::Errored);

                            let _ = self
                                .as_mut()
                                .events
                                .send_error(Error::UnexpectedEndOfStream);

                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            tracing::trace!(target: CONN, "No incoming commands");

                            tracing::trace!(target: CONN, "Pending");

                            return Poll::Pending;
                        }
                    }
                }
            }
        }
    }
}

impl<E: EventChannel, R: Delay + Timeout> NoSpawnConnectionBuilder<E, R> {
    /// Consumes the builder and creates a new [`Client`] along with the connection future and event stream (from raw parts).
    pub(crate) fn raw<F>(
        self,
        framed: F,
    ) -> (
        Client<R>,
        impl Stream<Item = E::Event> + Unpin + 'static,
        impl Future<Output = ()>,
    )
    where
        F: Stream<Item = Result<Command, DecodeError>>
            + for<'a> Sink<&'a Command, Error = EncodeError>,
    {
        let (connection, watch, actions, events, late_responses_dropped) =
            Connection::<_, E, R>::new(
                self.builder.enquire_link_interval,
                self.builder.enquire_link_response_timeout,
                self.builder.auto_enquire_link_response,
            );

        let client = Client::new(
            actions,
            self.builder.response_timeout,
            self.builder.check_interface_version,
            watch,
            late_responses_dropped,
        );

        (client, events, async move {
            let mut framed = std::pin::pin!(framed);

            let connection = connection.with_framed(&mut framed);

            // See comments on Connection struct to understand why we fuse the connection future.
            connection.fuse().await;

            tracing::debug!(target: "rusmppc::connection::tcp", "Shutting down stream");

            if let Err(err) = framed.close().await {
                tracing::error!(target: "rusmppc::connection::tcp", ?err, "Failed to shutdown stream");
            }
        })
    }
}

#[cfg(test)]
mod tests;
