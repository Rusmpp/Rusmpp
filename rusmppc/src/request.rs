use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use rusmpp::Command;
use tokio::sync::{mpsc, oneshot};

use crate::error::Error;

/// The identity of a request within one connection.
///
/// It exists to address cancellation cleanup: a request is identified by its own state cell
/// (see [`RequestCell`]), and the id only tells the connection which queued request a
/// cancelled caller is talking about.
pub(crate) type RequestId = u64;

/// The outcome of a request, as the connection reports it to the caller.
///
/// There is exactly one outcome channel per request, and it is written at most twice: the
/// write acknowledgement, then one terminal notification. The channel carries **notifications
/// only** — the server's response itself lives in the request's state cell until its
/// disposition is decided, so the cell (not the channel, and not the connection task) is the
/// single arbiter of the terminal outcome.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub(crate) enum Outcome {
    /// The write acknowledgement: the request has been handed to the transport and its
    /// flush completed. Not terminal — the response is still owed.
    Written { sequence_number: u32 },
    /// The server's response is committed to the request's cell: take it from there.
    ///
    /// Terminal when taken; if the caller is gone by then, the cell's drop or timeout path
    /// decides its disposition (the "response always wins" rule lives in the cell).
    ResponseReady,
    /// The request failed. Terminal.
    Failed(Error),
}

/// The sending half of a request's outcome channel, held by the connection (from the write
/// gate) and by the request itself (for the failures that end it).
pub(crate) type OutcomeSender = mpsc::Sender<Outcome>;

/// The receiving half of a request's outcome channel, held by the caller.
pub(crate) type OutcomeReceiver = mpsc::Receiver<Outcome>;

/// Capacity of a request's outcome channel: one write acknowledgement plus one terminal
/// notification, and nothing else.
///
/// The connection never blocks on it (it is always `try_send`), and a full channel is a
/// protocol violation — the notification is never dropped silently, it is counted through
/// `tracing`.
pub(crate) const OUTCOME_CHANNEL_CAPACITY: usize = 2;

/// Where a late response goes: the connection's late lane.
///
/// A response whose caller is gone is still owed to the application, so it is handed to the
/// connection and surfaced as an incoming event. The lane never retains the connection and a
/// forward never runs user code under the lane lock; it can briefly block behind the lane's
/// own short critical sections (it takes the lane's mutex — see `LateLane` for the lock
/// discipline), and a lane whose teardown has closed it makes the forward fail — that
/// failure is counted HERE, at the sender, which is what makes "delivered or counted"
/// exhaustive.
#[derive(Debug, Clone)]
pub(crate) struct LateHandle {
    lane: Arc<crate::connection::LateLane>,
    dropped: Arc<AtomicU64>,
}

impl LateHandle {
    pub(crate) fn new(lane: Arc<crate::connection::LateLane>, dropped: Arc<AtomicU64>) -> Self {
        Self { lane, dropped }
    }

    /// Hands a late response to the connection's late lane.
    ///
    /// A closed lane means the connection is gone: nothing can deliver the response any more,
    /// and the failure is counted rather than silent.
    pub(crate) fn forward(&self, command: Command) {
        if !self.lane.enqueue(command) {
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// The request's own state cell: the single arbiter of its terminal outcome.
///
/// The connection owns `Queued -> Written` (the write gate, immediately before
/// `start_send`); the caller owns `Queued -> Abandoned` (decided synchronously by the
/// future's guard or by its response timeout). Beyond that, the cell owns the response
/// payload once the peer has answered: whoever reaches the cell first decides whether the
/// response goes to the caller (the timeout that finds it staged takes it), to the late path
/// (the decoder that finds the request abandoned, or the drop that finds it unconsumed), or
/// nowhere at all (the caller consumed it). "Definitely not sent" is therefore decided by the
/// caller's own cell, never by asking a connection that may be gone or starved.
#[derive(Debug, Clone)]
pub(crate) struct RequestCell(Arc<Mutex<RequestCellInner>>);

#[derive(Debug)]
struct RequestCellInner {
    state: RequestState,
    /// The server's response, held until its disposition is decided.
    payload: Option<Command>,
    /// Where a late response goes; installed by the connection at the write gate.
    late: Option<LateHandle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestState {
    /// Waiting in the write queue. The caller may still abandon it, and the write gate will
    /// not write an abandoned request.
    Queued,
    /// Handed to the sink (`start_send` accepted it): the boundary. The bytes can no longer
    /// be retracted, so this request may reach the peer.
    Written { sequence_number: u32 },
    /// The caller gave up (its future was dropped, or its response timeout elapsed).
    ///
    /// `written` is the sequence number the request had reached when the caller gave up:
    /// `None` means it never left the queue — **definitely not sent** — and `Some` that it
    /// may already be in the peer's hands.
    Abandoned { written: Option<u32> },
}

/// What the caller learned when it abandoned its request (see [`RequestState`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AbandonOutcome {
    /// The request was still queued: it is definitely not sent, and retrying it is safe.
    NotWritten,
    /// The request had been handed to the sink: it may reach the peer.
    Written { sequence_number: u32 },
}

/// What settling a response timeout decided (see [`RequestCell::settle_timeout`]).
// The variant size difference is inherent (a response carries a full PDU); the enum is
// short-lived and sits on the caller's task stack, so indirection would only add a
// pointless allocation on the timeout path.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub(crate) enum TimeoutSettlement {
    /// A response was committed to the cell before the timeout settled: it wins, and the
    /// caller resolves with it.
    Response(Command),
    /// The timeout settled first: the request is abandoned, and a response committed after
    /// this goes to the late path instead.
    Abandoned(AbandonOutcome),
}

/// The verdict a request state carries: what the request had reached when it stopped.
fn resolve(state: RequestState) -> AbandonOutcome {
    match state {
        RequestState::Queued => AbandonOutcome::NotWritten,
        RequestState::Written { sequence_number } => AbandonOutcome::Written { sequence_number },
        RequestState::Abandoned { written: None } => AbandonOutcome::NotWritten,
        RequestState::Abandoned {
            written: Some(sequence_number),
        } => AbandonOutcome::Written { sequence_number },
    }
}

/// Marks a cell abandoned, returning what the request had reached. Idempotent: an
/// already-abandoned cell reports what the first abandonment decided.
fn mark_abandoned(state: &mut RequestState) -> AbandonOutcome {
    let outcome = resolve(*state);

    if !matches!(*state, RequestState::Abandoned { .. }) {
        *state = RequestState::Abandoned {
            written: match outcome {
                AbandonOutcome::NotWritten => None,
                AbandonOutcome::Written { sequence_number } => Some(sequence_number),
            },
        };
    }

    outcome
}

impl RequestCell {
    pub(crate) fn new() -> Self {
        Self(Arc::new(Mutex::new(RequestCellInner {
            state: RequestState::Queued,
            payload: None,
            late: None,
        })))
    }

    /// The write gate: `Queued -> Written`.
    ///
    /// `assign` runs inside the cell's lock and returns the sequence number to write the
    /// request with (the connection skips the numbers still reserved by another request).
    /// The transition is synchronous: there is no await between the check, the assignment,
    /// the registration and `start_send`, so a caller that abandons the request wins the
    /// race exactly when it reaches the cell first.
    ///
    /// `None` means the caller abandoned the request before the gate: nothing was assigned,
    /// and the request must be dropped without being registered or written.
    pub(crate) fn claim(&self, assign: impl FnOnce() -> u32) -> Option<u32> {
        let mut inner = self.0.lock().unwrap();

        match inner.state {
            RequestState::Queued => {
                let sequence_number = assign();

                inner.state = RequestState::Written { sequence_number };

                Some(sequence_number)
            }
            // Only the write gate claims, and it claims each request once; an abandoned
            // request must never be written.
            RequestState::Written { .. } | RequestState::Abandoned { .. } => None,
        }
    }

    /// Installs the late lane this cell's responses are forwarded to when their caller is
    /// gone. Called by the connection at the write gate, before `start_send`: a response can
    /// not exist before that point.
    pub(crate) fn install_late(&self, late: LateHandle) {
        self.0.lock().unwrap().late = Some(late);
    }

    /// Commits the server's response to the cell, atomically deciding its disposition.
    ///
    /// `Ok(())` means the response is stored for the caller. `Err(command)` means the caller
    /// gave up before this commit reached the cell: the response is handed back for the late
    /// path (the decoder surfaces it as an incoming event), so it is never lost into a
    /// receiver nobody reads.
    // The Err payload is the command itself (a full PDU); returning it is the point — the
    // alternative is an allocation on the abandoned-request path.
    #[allow(clippy::result_large_err)]
    pub(crate) fn commit_response(&self, command: Command) -> Result<(), Command> {
        let mut inner = self.0.lock().unwrap();

        if matches!(inner.state, RequestState::Abandoned { .. }) {
            return Err(command);
        }

        debug_assert!(
            inner.payload.is_none(),
            "a response is committed to a request at most once"
        );

        inner.payload = Some(command);

        Ok(())
    }

    /// Takes the stored response for the caller: consumption, after which nothing is owed.
    pub(crate) fn take_response(&self) -> Option<Command> {
        self.0.lock().unwrap().payload.take()
    }

    /// The sequence number of a stored, not-yet-consumed response, if one is stored.
    ///
    /// The raw API's write stage reads it to name the request it resolved on, without
    /// taking the response itself: ownership stays with the cell until the response future
    /// consumes it.
    pub(crate) fn stored_response_sequence_number(&self) -> Option<u32> {
        self.0
            .lock()
            .unwrap()
            .payload
            .as_ref()
            .map(Command::sequence_number)
    }

    /// The atomic timeout settlement: *the response wins if it was already committed*.
    ///
    /// One step, under the cell's lock: a stored response is taken and returned (the timeout
    /// never downgrades an answer the peer already gave); otherwise the request is abandoned,
    /// and every later commit goes to the late path.
    pub(crate) fn settle_timeout(&self) -> TimeoutSettlement {
        let mut inner = self.0.lock().unwrap();

        if let Some(command) = inner.payload.take() {
            return TimeoutSettlement::Response(command);
        }

        TimeoutSettlement::Abandoned(mark_abandoned(&mut inner.state))
    }

    /// The caller walked away: take an unconsumed response if one is stored — it is handed
    /// back for late routing — and mark the request abandoned either way. One atomic step,
    /// so a response committed concurrently either lands here (and is forwarded) or is
    /// routed late by the decoder; it can not fall between the two.
    pub(crate) fn abandon_or_take(&self) -> Option<Command> {
        let mut inner = self.0.lock().unwrap();

        let command = inner.payload.take();

        let _ = mark_abandoned(&mut inner.state);

        command
    }

    /// Hands a taken response to the late lane (see [`LateHandle::forward`]).
    pub(crate) fn forward_late(&self, command: Command) {
        let late = self.0.lock().unwrap().late.clone();

        if let Some(late) = late {
            late.forward(command);
        }
    }

    /// What the request had reached, without changing it.
    ///
    /// The verdict for a caller that can no longer get an outcome for its request (the
    /// connection dropped it or its registration): `NotWritten` means the request never
    /// left the queue — definitely not sent — and `Written` that it may be in the peer's
    /// hands.
    pub(crate) fn resolution(&self) -> AbandonOutcome {
        resolve(self.0.lock().unwrap().state)
    }

    /// Whether the caller has abandoned the request.
    ///
    /// An entry whose cell is abandoned is a reservation nobody waits on any more: the
    /// sequence number stays reserved until the response arrives (late) or the connection
    /// ends.
    pub(crate) fn is_abandoned(&self) -> bool {
        matches!(self.0.lock().unwrap().state, RequestState::Abandoned { .. })
    }
}

/// The connection's registration for a written request: what it still owes, and to whom.
#[derive(Debug)]
pub(crate) struct PendingEntry {
    /// The request this entry belongs to. The entry and the request can be told apart even
    /// after the sequence number has been reused.
    pub(crate) id: RequestId,
    /// The request's one outcome channel: the write acknowledgement, and then the terminal
    /// notification.
    pub(crate) outcome: OutcomeSender,
    /// The request's state cell: the arbiter its response is committed to.
    pub(crate) cell: RequestCell,
}

#[derive(Debug)]
pub enum Request {
    /// Requests for which we are waiting for a response from the server.
    ///
    /// These requests are stored in the connection's pending requests map.
    Registered(RegisteredRequest),
    /// Requests for which we are `not` waiting for a response from the server.
    ///
    /// These requests are `not` stored in the connection's pending requests map.
    Unregistered(UnregisteredRequest),
    /// Request issued by the connection itself to send a command to the server.
    ///
    /// These requests are `not` stored in the connection's pending requests map, carry no
    /// state cell (there is no caller that could cancel them) and are not acknowledged.
    Obligated(ObligatedRequest),
}

impl Request {
    pub fn command(&self) -> &Command {
        match self {
            Request::Registered(request) => &request.command,
            Request::Unregistered(request) => &request.command,
            Request::Obligated(request) => &request.command,
        }
    }

    /// The request's id; `None` for obligated requests (they have no caller).
    pub(crate) fn id(&self) -> Option<RequestId> {
        match self {
            Request::Registered(request) => Some(request.id),
            Request::Unregistered(request) => Some(request.id),
            Request::Obligated(_) => None,
        }
    }

    /// Whether this request takes a reserved sequence-number slot at the write gate: a
    /// registered request. Nothing else does — a manually sent response PDU
    /// (`deliver_sm_resp`, `enquire_link_resp`, …) is written under a sequence number that
    /// nothing is registered for, so the admission bound neither counts nor refuses it.
    pub(crate) fn registers(&self) -> bool {
        matches!(self, Request::Registered(_))
    }

    /// Whether the caller of this request has abandoned it (see [`RequestCell`]).
    pub(crate) fn is_abandoned(&self) -> bool {
        match self {
            Request::Registered(request) => request.cell.is_abandoned(),
            Request::Unregistered(request) => request.cell.is_abandoned(),
            Request::Obligated(_) => false,
        }
    }

    /// The write gate: claims the request, assigns its sequence number, and registers it —
    /// in one synchronous step, after the sink accepted the write.
    ///
    /// `None` means the caller abandoned the request before the gate: nothing was assigned
    /// or registered, and the request must be dropped without being written. The gate
    /// renumbers the request when its proposed number is still reserved by another request
    /// (a live registration or an unresolved one), so a pending response can never be
    /// delivered to the wrong waiter.
    pub(crate) fn claim(&mut self, pending: &mut BTreeMap<u32, PendingEntry>) -> Option<u32> {
        match self {
            Request::Registered(request) => {
                let proposed = request.command.sequence_number();

                let sequence_number = request
                    .cell
                    .claim(|| next_free_sequence_number(pending, proposed))?;

                if sequence_number != proposed {
                    tracing::debug!(
                        proposed,
                        sequence_number,
                        "the proposed sequence number is still reserved; writing with a free one"
                    );

                    request.command.sequence_number = sequence_number;
                }

                pending.insert(
                    sequence_number,
                    PendingEntry {
                        id: request.id,
                        outcome: request.outcome.clone(),
                        cell: request.cell.clone(),
                    },
                );

                Some(sequence_number)
            }
            Request::Unregistered(request) => {
                let proposed = request.command.sequence_number();

                // An unregistered request is never stored in the map: its proposed number is
                // echoed from the server's (`deliver_sm_resp`, `enquire_link_resp`, …) or
                // allocated by the client, and either way nothing is registered under it, so
                // it is not renumbered and not checked against the table.
                let sequence_number = request.cell.claim(|| proposed)?;

                Some(sequence_number)
            }
            Request::Obligated(request) => Some(request.command.sequence_number()),
        }
    }

    /// Reports a failure the request reached before its write gate: nothing was registered,
    /// so the request's own outcome channel carries it.
    pub(crate) fn send_failed(self, error: Error) {
        match self {
            Request::Registered(request) => request.send_failed(error),
            Request::Unregistered(request) => request.send_failed(error),
            Request::Obligated(_) => {}
        }
    }
}

/// The next free sequence number for a request the client proposed `proposed` for.
///
/// The client allocates the odd numbers in `1..=0x7FFFFFFF`, stepping by two (the
/// connection's own even numbers are never registered here), so the walk stays in the same
/// class and only skips the numbers this connection still reserves — every unresolved
/// registration, live or abandoned alike. At most one candidate per occupied entry is
/// examined: the walk can not loop.
fn next_free_sequence_number(pending: &BTreeMap<u32, PendingEntry>, proposed: u32) -> u32 {
    let mut candidate = proposed;

    for _ in 0..=pending.len() {
        if !pending.contains_key(&candidate) {
            return candidate;
        }

        candidate = next_in_class(candidate);
    }

    // Unreachable while the map holds at most `pending.len()` distinct occupied numbers and
    // the walk visits that many distinct candidates; a bound that never lies is worse than
    // a bound that does (this is a debug-assert, not a panic).
    debug_assert!(false, "no free sequence number found for {proposed}");

    candidate
}

/// The next sequence number in the client's class: odd numbers, wrapping back to `1` inside
/// the SMPP range (the same step the client's own allocator takes).
fn next_in_class(sequence_number: u32) -> u32 {
    if sequence_number >= crate::client::MAX_SEQUENCE_NUMBER - 1 {
        1
    } else {
        sequence_number + 2
    }
}

#[derive(Debug)]
pub struct RegisteredRequest {
    /// The request's id (see [`RequestId`]).
    pub id: RequestId,
    pub command: Command,
    /// The request's state cell (see [`RequestCell`]).
    pub cell: RequestCell,
    /// The request's one outcome channel: the write acknowledgement, then the terminal
    /// notification. The sender leaves the request at the write gate (a clone stays in the
    /// entry), and every failure before the gate uses the request's own sender.
    pub outcome: OutcomeSender,
}

impl RegisteredRequest {
    /// The write acknowledgement: the request has been handed to the transport and flushed.
    ///
    /// One *message*, not the terminal outcome: the response (or the failure that ends the
    /// request) is still owed. Sent once per request, when the pending write's flush
    /// completes — or never, when the response arrives first (the caller then reads the
    /// response from the cell).
    pub(crate) fn send_written(&self) {
        let _ = self.outcome.try_send(Outcome::Written {
            sequence_number: self.command.sequence_number(),
        });
    }

    /// Reports a failure the request reached before it was written.
    pub(crate) fn send_failed(self, error: Error) {
        let _ = self.outcome.try_send(Outcome::Failed(error));
    }
}

impl RegisteredRequest {
    pub fn new(id: RequestId, command: Command) -> (Self, OutcomeReceiver) {
        let (outcome, receiver) = mpsc::channel(OUTCOME_CHANNEL_CAPACITY);

        (
            Self {
                id,
                command,
                cell: RequestCell::new(),
                outcome,
            },
            receiver,
        )
    }
}

#[derive(Debug)]
pub struct UnregisteredRequest {
    /// The request's id (see [`RequestId`]).
    pub id: RequestId,
    pub command: Command,
    /// The request's state cell (see [`RequestCell`]).
    pub cell: RequestCell,
    /// The request's one outcome channel: the write acknowledgement, then the terminal
    /// notification.
    pub outcome: OutcomeSender,
}

impl UnregisteredRequest {
    pub fn new(id: RequestId, command: Command) -> (Self, OutcomeReceiver) {
        let (outcome, receiver) = mpsc::channel(OUTCOME_CHANNEL_CAPACITY);

        (
            Self {
                id,
                command,
                cell: RequestCell::new(),
                outcome,
            },
            receiver,
        )
    }

    /// The write acknowledgement (see [`RegisteredRequest::send_written`]).
    pub(crate) fn send_written(&self) {
        let _ = self.outcome.try_send(Outcome::Written {
            sequence_number: self.command.sequence_number(),
        });
    }

    /// Reports a failure the request reached before it was written.
    pub(crate) fn send_failed(self, error: Error) {
        let _ = self.outcome.try_send(Outcome::Failed(error));
    }
}

#[derive(Debug)]
pub struct ObligatedRequest {
    pub command: Command,
}

impl ObligatedRequest {
    pub const fn new(command: Command) -> Self {
        Self { command }
    }
}

#[derive(Debug)]
pub struct CloseRequest {
    /// ack result means that the connection started processing the close request.
    pub ack: oneshot::Sender<()>,
}

impl CloseRequest {
    pub fn new() -> (Self, oneshot::Receiver<()>) {
        let (ack, rx) = oneshot::channel();

        (Self { ack }, rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(sequence_number: u32) -> Command {
        use rusmpp::{CommandStatus, Pdu};

        Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(sequence_number)
            .pdu(Pdu::DeliverSmResp(Default::default()))
    }

    /// The arbitration table: a response committed before the timeout settles wins — the
    /// timeout takes it and returns it, and nothing is left for a later drop.
    #[test]
    fn a_committed_response_wins_over_a_later_timeout_settlement() {
        let cell = RequestCell::new();

        assert_eq!(cell.claim(|| 1), Some(1));
        assert!(cell.commit_response(response(1)).is_ok());

        match cell.settle_timeout() {
            TimeoutSettlement::Response(command) => assert_eq!(command.sequence_number(), 1),
            other => panic!("the committed response must win, got {other:?}"),
        }

        assert!(
            cell.abandon_or_take().is_none(),
            "the response was consumed by the timeout: nothing is left to forward"
        );
    }

    /// The other order: the timeout settles first, so the commit is handed back for the late
    /// path — it is never stored for a caller that is already gone.
    #[test]
    fn a_settled_timeout_sends_a_later_commit_to_the_late_path() {
        let cell = RequestCell::new();

        assert_eq!(cell.claim(|| 1), Some(1));

        match cell.settle_timeout() {
            TimeoutSettlement::Abandoned(AbandonOutcome::Written { sequence_number }) => {
                assert_eq!(sequence_number, 1);
            }
            other => panic!("the timeout must settle the request as written, got {other:?}"),
        }

        assert!(cell.commit_response(response(1)).is_err());
    }

    /// A stored response that the caller never consumed is handed back when the caller drops
    /// (it is routed late), and a consumed one leaves nothing behind.
    #[test]
    fn a_stored_response_is_returned_to_the_drop_unless_it_was_consumed() {
        let cell = RequestCell::new();

        assert_eq!(cell.claim(|| 1), Some(1));
        assert!(cell.commit_response(response(1)).is_ok());

        match cell.abandon_or_take() {
            Some(command) => assert_eq!(command.sequence_number(), 1),
            None => panic!("the unconsumed response must be handed back for late routing"),
        }

        assert!(cell.abandon_or_take().is_none());

        // The consumed variant: take first, then the drop finds nothing.
        let cell = RequestCell::new();

        assert_eq!(cell.claim(|| 3), Some(3));
        assert!(cell.commit_response(response(3)).is_ok());
        assert!(cell.take_response().is_some());
        assert!(cell.abandon_or_take().is_none());
    }

    /// A timeout on a request that was never written (or never claimed at all) reports the
    /// "definitely not sent" verdict, and a later commit still goes late.
    #[test]
    fn a_settled_timeout_on_an_unwritten_request_reports_not_written() {
        let cell = RequestCell::new();

        match cell.settle_timeout() {
            TimeoutSettlement::Abandoned(AbandonOutcome::NotWritten) => {}
            other => panic!("a queued request's timeout is definitely not sent, got {other:?}"),
        }

        assert!(cell.commit_response(response(1)).is_err());
    }
}
