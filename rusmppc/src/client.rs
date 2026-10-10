#![allow(clippy::result_large_err)]
// This warning is triggered on `extract` macro that extracts a specific `Pdu` variant from a generic `Pdu`.
// The `Ok` variant is the specific `Pdu` variant, while the `Err` variant is the generic `Pdu` that can be large.

use std::{
    fmt::Debug,
    sync::{
        Arc,
        atomic::{AtomicU32, AtomicU64, Ordering},
    },
    time::Duration,
};

use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    command::CommandParts,
    pdus::{
        BindReceiver, BindReceiverResp, BindTransceiver, BindTransceiverResp, BindTransmitter,
        BindTransmitterResp, BroadcastSm, BroadcastSmResp, CancelBroadcastSm, CancelSm, DataSm,
        DataSmResp, DeliverSmResp, QueryBroadcastSm, QueryBroadcastSmResp, QuerySm, QuerySmResp,
        ReplaceSm, SubmitMulti, SubmitMultiResp, SubmitSm, SubmitSmResp,
    },
    values::InterfaceVersion,
};
use tokio::sync::{mpsc::UnboundedSender, watch};

use crate::{
    AbandonOutcome, Action, CloseRequest, CommandExt, DefaultTokioConnectionBuilder,
    DefaultWasmConnectionBuilder, Outcome, OutcomeReceiver, PendingResponses, RegisteredRequest,
    RequestCell, RequestFutureGuard, RequestId, TimeoutSettlement, UnregisteredRequest,
    error::Error,
    error::NotSentReason,
    runtime_::{Timeout, tokio::Tokio, wasm::Wasm},
};

const TARGET: &str = "rusmppc::client";

/// The highest sequence number in the SMPP range: `0x00000001..=0x7FFFFFFF` (0 is not a
/// valid sequence number).
///
/// The client allocates the odd numbers in that range and the connection its own even
/// ones, so the two allocators on one connection never collide with each other. A wrap
/// can still land on a number a live request — or an abandoned one's tombstone — still
/// reserves; the connection does not refuse such a request: its write gate assigns the
/// number actually written, skipping to the next free number in the same class.
pub(crate) const MAX_SEQUENCE_NUMBER: u32 = 0x7fff_ffff;

/// `SMPP` Client.
///
/// The client is a handle to communicate with the `SMPP` server through a managed connection in the background.
pub struct Client<T = Tokio> {
    inner: Arc<ClientInner<T>>,
}

impl<T> Clone for Client<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Debug for Client<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish()
    }
}

impl Client<Tokio> {
    /// Creates a new `SMPP` connection builder.
    ///
    /// See [`DefaultTokioConnectionBuilder::new`] for more details.
    pub fn builder() -> DefaultTokioConnectionBuilder {
        DefaultTokioConnectionBuilder::new()
    }

    /// Creates a new `SMPP` connection builder.
    ///
    /// See [`DefaultTokioConnectionBuilder::new`] for more details.
    pub fn builder_tokio() -> DefaultTokioConnectionBuilder {
        DefaultTokioConnectionBuilder::new_tokio()
    }
}

impl Client<Wasm> {
    /// Creates a new `SMPP` connection builder.
    ///
    /// See [`DefaultWasmConnectionBuilder::new_wasm`] for more details.
    pub fn builder_wasm() -> DefaultWasmConnectionBuilder {
        DefaultWasmConnectionBuilder::new_wasm()
    }
}

impl<T: Timeout> Client<T> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        actions: UnboundedSender<Action>,
        response_timeout: Option<Duration>,
        check_interface_version: bool,
        watch: watch::Sender<()>,
        late_responses_dropped: Arc<AtomicU64>,
    ) -> Self {
        Self {
            inner: Arc::new(ClientInner::new(
                actions,
                response_timeout,
                check_interface_version,
                watch,
                late_responses_dropped,
            )),
        }
    }

    /// How many late responses could not be delivered to the event stream.
    ///
    /// A response whose caller is gone is still owed to the application: the request's
    /// cell forwards it to the connection's late lane, and the connection surfaces it as
    /// an incoming event. The forward can fail on either side of that lane — the lane's
    /// connection task is gone (the send fails), or the event channel refuses the surfaced
    /// command — and such a loss is counted here rather than dropped silently. Monotonic
    /// for the life of the connection.
    pub fn late_responses_dropped(&self) -> u64 {
        self.inner.late_responses_dropped.load(Ordering::Relaxed)
    }

    /// Sends a [`BindTransmitter`] command to the server and waits for a successful [`BindTransmitterResp`].
    pub async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<BindTransmitterResp, Error> {
        self.registered_request().bind_transmitter(bind).await
    }

    /// Sends a [`BindReceiver`] command to the server and waits for a successful [`BindReceiverResp`].
    pub async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<BindReceiverResp, Error> {
        self.registered_request().bind_receiver(bind).await
    }

    /// Sends a [`BindTransceiver`] command to the server and waits for a successful [`BindTransceiverResp`].
    pub async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<BindTransceiverResp, Error> {
        self.registered_request().bind_transceiver(bind).await
    }

    /// Sends a [`BroadcastSm`] command to the server and waits for a successful [`BroadcastSmResp`].
    pub async fn broadcast_sm(
        &self,
        broadcast_sm: impl Into<BroadcastSm>,
    ) -> Result<BroadcastSmResp, Error> {
        self.registered_request().broadcast_sm(broadcast_sm).await
    }

    /// Sends a [`CancelBroadcastSm`] command to the server and waits for a successful [`CancelBroadcastSmResp`](Pdu::CancelBroadcastSmResp).
    pub async fn cancel_broadcast_sm(
        &self,
        cancel_broadcast_sm: impl Into<CancelBroadcastSm>,
    ) -> Result<(), Error> {
        self.registered_request()
            .cancel_broadcast_sm(cancel_broadcast_sm)
            .await
    }

    /// Sends a [`CancelSm`] command to the server and waits for a successful [`CancelSmResp`](Pdu::CancelSmResp).
    pub async fn cancel_sm(&self, cancel_sm: impl Into<CancelSm>) -> Result<(), Error> {
        self.registered_request().cancel_sm(cancel_sm).await
    }

    /// Sends a [`DataSm`] command to the server and waits for a successful [`DataSmResp`].
    pub async fn data_sm(&self, data_sm: impl Into<DataSm>) -> Result<DataSmResp, Error> {
        self.registered_request().data_sm(data_sm).await
    }

    /// Sends a [`DataSmResp`] command to the server.
    pub async fn data_sm_resp(
        &self,
        sequence_number: u32,
        data_sm_resp: impl Into<DataSmResp>,
    ) -> Result<(), Error> {
        self.unregistered_request()
            .data_sm_resp(sequence_number, data_sm_resp)
            .await
    }

    /// Sends a [`DeliverSmResp`] command to the server.
    pub async fn deliver_sm_resp(
        &self,
        sequence_number: u32,
        deliver_sm_resp: impl Into<DeliverSmResp>,
    ) -> Result<(), Error> {
        self.unregistered_request()
            .deliver_sm_resp(sequence_number, deliver_sm_resp)
            .await
    }

    /// Sends a [`QueryBroadcastSm`] command to the server and waits for a successful [`QueryBroadcastSmResp`].
    pub async fn query_broadcast_sm(
        &self,
        query_broadcast_sm: impl Into<QueryBroadcastSm>,
    ) -> Result<QueryBroadcastSmResp, Error> {
        self.registered_request()
            .query_broadcast_sm(query_broadcast_sm)
            .await
    }

    /// Sends a [`QuerySm`] command to the server and waits for a successful [`QuerySmResp`].
    pub async fn query_sm(&self, query_sm: impl Into<QuerySm>) -> Result<QuerySmResp, Error> {
        self.registered_request().query_sm(query_sm).await
    }

    /// Sends a [`ReplaceSm`] command to the server and waits for a successful [`ReplaceSmResp`](Pdu::ReplaceSmResp).
    pub async fn replace_sm(&self, replace_sm: impl Into<ReplaceSm>) -> Result<(), Error> {
        self.registered_request().replace_sm(replace_sm).await
    }

    /// Sends a [`SubmitMulti`] command to the server and waits for a successful [`SubmitMultiResp`].
    pub async fn submit_multi(
        &self,
        submit_multi: impl Into<SubmitMulti>,
    ) -> Result<SubmitMultiResp, Error> {
        self.registered_request().submit_multi(submit_multi).await
    }

    /// Sends a [`SubmitSm`] command to the server and waits for a successful [`SubmitSmResp`].
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
        self.registered_request().submit_sm(submit_sm).await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server and waits for a successful [`UnbindResp`](Pdu::UnbindResp).
    pub async fn unbind(&self) -> Result<(), Error> {
        self.registered_request().unbind().await
    }

    /// Sends an [`UnbindResp`](Pdu::UnbindResp) command to the server.
    pub async fn unbind_resp(&self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request()
            .unbind_resp(sequence_number)
            .await
    }

    /// Sends an [`EnquireLink`](Pdu::EnquireLink) command to the server and waits for a successful [`EnquireLinkResp`](Pdu::EnquireLinkResp).
    pub async fn enquire_link(&self) -> Result<(), Error> {
        self.registered_request().enquire_link().await
    }

    /// Sends an [`EnquireLinkResp`](Pdu::EnquireLinkResp) command to the server.
    pub async fn enquire_link_resp(&self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request()
            .enquire_link_resp(sequence_number)
            .await
    }

    /// Sends a [`GenericNack`](Pdu::GenericNack) command to the server.
    pub async fn generic_nack(&self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request()
            .generic_nack(sequence_number)
            .await
    }

    /// Test-only: seeds the sequence allocator (see [`ClientInner::next_sequence_number`]).
    #[cfg(test)]
    pub(crate) fn seed_sequence_number(&self, sequence_number: u32) {
        self.inner
            .sequence_number
            .store(sequence_number, Ordering::Relaxed);
    }

    /// Closes the connection.
    ///
    /// This method completes, when the connection has registered the close request.
    /// The connection will stop reading from the server, stop time keeping, close the requests channel, flush pending requests and terminate.
    ///
    /// After calling this method, clients can no longer send requests to the server.
    pub async fn close(&self) -> Result<(), Error> {
        self.inner.close().await
    }

    /// Checks if the connection is closed.
    ///
    /// # Note
    ///
    /// If the connection is not closed, this does not mean that it is active.
    /// The connection may be in the process of closing.
    ///
    /// To check if the connection is active, use [`Client::is_active()`].
    pub fn is_closed(&self) -> bool {
        self.inner.watch.is_closed()
    }

    /// Completes when the connection is closed.
    pub async fn closed(&self) {
        self.inner.watch.closed().await
    }

    /// Closes the connection and waits for it to terminate.
    pub async fn close_and_wait(&self) -> Result<(), Error> {
        self.close().await?;
        self.closed().await;

        Ok(())
    }

    /// Checks if the connection is active.
    ///
    /// The connection is considered active if:
    ///  - [`Client::close()`] was never called.
    ///  - The connection did not encounter an error.
    ///  - The connection can receive requests form the client.
    ///
    /// # Note
    ///
    /// If the connection is not active, this does not mean that it is closed.
    /// The connection may be in the process of closing.
    ///
    /// To check if the connection is closed, use [`Client::is_closed()`].
    pub fn is_active(&self) -> bool {
        // If the connection is not active, closing or errored,
        // it will close the actions channel and stop receiving actions, this call would fail.
        self.inner.actions.send(Action::Ping).is_ok()
    }

    /// Returns a vector of pending responses.
    pub async fn pending_responses(&self) -> Result<Vec<u32>, Error> {
        let (pending_responses, ack) = PendingResponses::new();

        self.inner
            .actions
            .send(Action::PendingResponses(pending_responses))
            .map_err(|_| Error::ConnectionClosed)?;

        ack.await.map_err(|_| Error::ConnectionClosed)?
    }

    /// Sets the command status for the next request.
    pub const fn status(&'_ self, status: CommandStatus) -> UnregisteredRequestBuilder<'_, T> {
        self.unregistered_request().status(status)
    }

    /// Sets the response timeout for the next request.
    pub fn response_timeout(&'_ self, timeout: Duration) -> RegisteredRequestBuilder<'_, T> {
        self.registered_request().response_timeout(timeout)
    }

    /// Disables the response timeout for the next request.
    pub fn no_response_timeout(&'_ self) -> RegisteredRequestBuilder<'_, T> {
        self.registered_request().no_response_timeout()
    }

    /// Sends a request without waiting for a response.
    pub const fn no_wait(&'_ self) -> NoWaitRequestBuilder<'_, T> {
        self.no_wait_request()
    }

    /// Sends a raw request to the server.
    pub fn raw(&'_ self) -> RawRegisteredRequestBuilder<'_, T> {
        self.raw_request()
    }

    const fn unregistered_request(&'_ self) -> UnregisteredRequestBuilder<'_, T> {
        UnregisteredRequestBuilder::new(self, CommandStatus::EsmeRok)
    }

    fn registered_request(&'_ self) -> RegisteredRequestBuilder<'_, T> {
        RegisteredRequestBuilder::new(self, CommandStatus::EsmeRok)
    }

    const fn no_wait_request(&'_ self) -> NoWaitRequestBuilder<'_, T> {
        NoWaitRequestBuilder::new(self, CommandStatus::EsmeRok)
    }

    fn raw_request(&'_ self) -> RawRegisteredRequestBuilder<'_, T> {
        RawRegisteredRequestBuilder::new(self, CommandStatus::EsmeRok)
    }
}

#[derive(Debug)]
struct ClientInner<T = Tokio> {
    actions: UnboundedSender<Action>,
    response_timeout: Option<Duration>,
    sequence_number: AtomicU32,
    /// The id of the next request (see [`RequestId`]): the address its cancellation
    /// cleanup hint uses.
    request_id: AtomicU64,
    check_interface_version: bool,
    watch: watch::Sender<()>,
    /// Late responses (their caller is gone) that could not be delivered to the event
    /// stream. Read through [`Client::late_responses_dropped`]: the loss happens at either
    /// end of the late lane, so it must be observable outside the connection.
    late_responses_dropped: Arc<AtomicU64>,
    _t: std::marker::PhantomData<T>,
}

impl<T: Timeout> ClientInner<T> {
    #[allow(clippy::too_many_arguments)]
    const fn new(
        actions: UnboundedSender<Action>,
        response_timeout: Option<Duration>,
        check_interface_version: bool,
        watch: watch::Sender<()>,
        late_responses_dropped: Arc<AtomicU64>,
    ) -> Self {
        Self {
            actions,
            response_timeout,
            sequence_number: AtomicU32::new(1),
            request_id: AtomicU64::new(0),
            check_interface_version,
            watch,
            late_responses_dropped,
            _t: std::marker::PhantomData,
        }
    }

    /// The id of the next request on this connection (see [`RequestId`]).
    fn next_request_id(&self) -> RequestId {
        self.request_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Allocates the next odd sequence number, inside the SMPP range.
    ///
    /// The client owns the odd numbers (`1, 3, … , 0x7FFFFFFF`) and the connection its
    /// own even ones, so the two allocators on one connection never collide with each
    /// other. A stored value consumed by a wrap (or seeded outside the range in tests)
    /// restarts the cycle at 1. The restarted number is only a proposal: the connection's
    /// write gate assigns the number actually written, skipping the ones still reserved by
    /// a live request or a tombstone — a wrap can never replace a live registration or
    /// leave the range.
    fn next_sequence_number(&self) -> u32 {
        loop {
            let current = self.sequence_number.load(Ordering::Relaxed);

            let allocated = if current == 0 || current > MAX_SEQUENCE_NUMBER {
                1
            } else {
                current
            };

            let next = if allocated >= MAX_SEQUENCE_NUMBER - 1 {
                1
            } else {
                allocated + 2
            };

            if self
                .sequence_number
                .compare_exchange_weak(current, next, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return allocated;
            }
        }
    }

    async fn close(&self) -> Result<(), Error> {
        let (request, ack) = CloseRequest::new();

        self.actions
            .send(Action::Close(request))
            .map_err(|_| Error::ConnectionClosed)?;

        ack.await.map_err(|_| Error::ConnectionClosed)
    }

    /// Queues a registered request and returns what the caller holds while it is in flight:
    /// the request's one outcome channel, its state cell (the cancellation authority) and
    /// its id (the cleanup hint's address).
    async fn send_registered(
        &self,
        command: Command,
    ) -> Result<(OutcomeReceiver, RequestCell, RequestId), Error> {
        let sequence_number = command.sequence_number();
        let status = command.status();
        let id = command.id();

        tracing::trace!(target: TARGET, sequence_number, ?status, ?id, "Sending request");

        let request_id = self.next_request_id();
        let (request, outcome) = RegisteredRequest::new(request_id, command);
        let cell = request.cell.clone();

        self.actions
            .send(Action::registered_request(request))
            .map_err(|_| Error::not_sent(NotSentReason::ConnectionClosed))?;

        Ok((outcome, cell, request_id))
    }

    /// Awaits the terminal outcome, applying the caller's response timeout.
    ///
    /// The timeout is measured from queueing: the request is registered (or refused) no
    /// later than the write, so a stalled flush can not postpone the deadline. When the
    /// timeout wins, the request is abandoned synchronously — the write gate will not write
    /// it any more — and the verdict follows the cell: a request that had been handed to
    /// the sink may be out, one that was still queued never left.
    async fn await_terminal_with_timeout(
        &self,
        receiver: &mut OutcomeReceiver,
        cell: &RequestCell,
        id: RequestId,
        response_timeout: Option<Duration>,
    ) -> Result<Command, Error> {
        let terminal = await_terminal(receiver, cell);

        match response_timeout {
            None => terminal.await,
            Some(timeout) => match T::timeout(timeout, terminal).await {
                Some(result) => result,
                None => {
                    // The atomic settlement: a response committed to the cell before this
                    // decision wins — the caller gets its response, never a timeout for a
                    // request the peer already answered. Only when nothing is stored does
                    // the timeout abandon the request, and every later commit goes to the
                    // late path instead.
                    match cell.settle_timeout() {
                        TimeoutSettlement::Response(command) => Ok(command),
                        TimeoutSettlement::Abandoned(abandoned) => {
                            // A cleanup hint only: the cell already decided, the gate
                            // refuses a still-queued request by itself, and a written
                            // request's reservation stays until its late reply arrives
                            // or the connection ends.
                            let _ = self.actions.send(Action::Cancel(id));

                            Err(match abandoned {
                                // The request was handed to the transport: it may be out.
                                // A retry can duplicate — this stays a "maybe sent" verdict.
                                AbandonOutcome::Written { sequence_number } => {
                                    Error::response_timeout(sequence_number, timeout)
                                }
                                // It never left the queue: definitely not sent, safe to
                                // retry.
                                AbandonOutcome::NotWritten => {
                                    Error::not_sent(NotSentReason::Timeout)
                                }
                            })
                        }
                    }
                }
            },
        }
    }
}

/// Waits for the terminal outcome of a queued request.
///
/// The write acknowledgement is not terminal: the response (or the failure that ends the
/// request) is still owed, and a response that arrived first wins over any later failure. A
/// channel closed without a terminal outcome means the connection dropped the request or
/// its registration; the request's own cell says what that meant.
async fn await_terminal(
    receiver: &mut OutcomeReceiver,
    cell: &RequestCell,
) -> Result<Command, Error> {
    loop {
        match receiver.recv().await {
            Some(Outcome::Written { .. }) => continue,
            Some(Outcome::ResponseReady) => {
                // The response itself lives in the cell, never in the channel: take it
                // from where the commit left it. Nothing to take means another arm of
                // this request settled it (the response-timeout settlement is atomic
                // with the commit) — keep waiting for a terminal it will provide.
                if let Some(command) = cell.take_response() {
                    return Ok(command);
                }
            }
            Some(Outcome::Failed(error)) => return Err(error),
            None => return Err(closed_without_outcome(cell)),
        }
    }
}

/// The verdict for a request whose outcome channel closed without a terminal outcome: the
/// connection dropped the request (or its registration) before anything resolved it.
///
/// The cell says what the request had reached: a request the write gate never claimed
/// definitely did not leave the queue, and retrying it can not duplicate; one that had
/// been handed to the transport may be out, so it stays a conservative "maybe sent".
fn closed_without_outcome(cell: &RequestCell) -> Error {
    match cell.resolution() {
        AbandonOutcome::Written { .. } => Error::ConnectionClosed,
        AbandonOutcome::NotWritten => {
            Error::not_sent(crate::error::NotSentReason::ConnectionClosed)
        }
    }
}

/// Builder for creating an unregistered requests.
///
/// Unregistered requests are requests that do not expect a response from the server, such as [`EnquireLinkResp`](Pdu::EnquireLinkResp), [`DeliverSmResp`](Pdu::DeliverSmResp), and [`UnbindResp`](Pdu::UnbindResp).
#[derive(Debug)]
pub struct UnregisteredRequestBuilder<'a, T> {
    client: &'a Client<T>,
    status: CommandStatus,
}

impl<'a, T: Timeout> UnregisteredRequestBuilder<'a, T> {
    const fn new(client: &'a Client<T>, status: CommandStatus) -> Self {
        Self { client, status }
    }

    fn registered_request(&'_ self) -> RegisteredRequestBuilder<'_, T> {
        RegisteredRequestBuilder::new(self.client, self.status)
    }

    const fn no_wait_request(&'_ self) -> NoWaitRequestBuilder<'_, T> {
        NoWaitRequestBuilder::new(self.client, self.status)
    }

    /// Sets the command status for the next request.
    pub const fn status(mut self, status: CommandStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the response timeout for the next request.
    pub fn response_timeout(&'_ self, timeout: Duration) -> RegisteredRequestBuilder<'_, T> {
        self.registered_request().response_timeout(timeout)
    }

    /// Disables the response timeout for the next request.
    pub fn no_response_timeout(&'_ self) -> RegisteredRequestBuilder<'_, T> {
        self.registered_request().no_response_timeout()
    }

    /// Sends a request without waiting for a response.
    pub const fn no_wait(&'_ self) -> NoWaitRequestBuilder<'_, T> {
        self.no_wait_request()
    }

    async fn unregistered_request(
        self,
        pdu: impl Into<Pdu>,
        sequence_number: u32,
    ) -> Result<(), Error> {
        let command = Command::builder()
            .status(self.status)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let sequence_number = command.sequence_number();
        let status = command.status();
        let id = command.id();

        tracing::trace!(target: TARGET, sequence_number, ?status, ?id, "Sending request");

        let request_id = self.client.inner.next_request_id();
        let (request, outcome) = UnregisteredRequest::new(request_id, command);
        let cell = request.cell.clone();

        if self
            .client
            .inner
            .actions
            .send(Action::unregistered_request(request))
            .is_err()
        {
            // The connection's action channel is gone: nothing was queued and nothing was
            // written, so this is a definite "not sent", not a closed-connection outcome.
            return Err(Error::not_sent(NotSentReason::ConnectionClosed));
        }

        self.wait_for_ack(outcome, cell, request_id, sequence_number, status, id)
            .await
    }

    /// Awaits a written request's write acknowledgement.
    ///
    /// An unregistered request is not waiting for a response, so there is no timeout here:
    /// the future resolves with the acknowledgement, or with the failure that ended the
    /// request before it was written.
    async fn wait_for_ack(
        self,
        mut outcome: OutcomeReceiver,
        cell: RequestCell,
        request_id: RequestId,
        sequence_number: u32,
        status: CommandStatus,
        id: CommandId,
    ) -> Result<(), Error> {
        tracing::trace!(target: TARGET, sequence_number, ?status, ?id, "Waiting for the write acknowledgement");

        let write_stage = async {
            loop {
                match outcome.recv().await {
                    Some(Outcome::Written { .. }) => return Ok(()),
                    // Unreachable: nothing is registered under an unregistered request's
                    // sequence number, so no response is ever routed to it.
                    Some(Outcome::ResponseReady) => continue,
                    Some(Outcome::Failed(error)) => return Err(error),
                    None => return Err(closed_without_outcome(&cell)),
                }
            }
        };

        RequestFutureGuard::new(
            &self.client.inner.actions,
            request_id,
            cell.clone(),
            write_stage,
        )
        .await
    }

    /// Sends a [`DataSmResp`] command to the server.
    pub async fn data_sm_resp(
        self,
        sequence_number: u32,
        data_sm_resp: impl Into<DataSmResp>,
    ) -> Result<(), Error> {
        self.unregistered_request(data_sm_resp.into(), sequence_number)
            .await
    }

    /// Sends a [`DeliverSmResp`] command to the server.
    pub async fn deliver_sm_resp(
        self,
        sequence_number: u32,
        deliver_sm_resp: impl Into<DeliverSmResp>,
    ) -> Result<(), Error> {
        self.unregistered_request(deliver_sm_resp.into(), sequence_number)
            .await
    }

    /// Sends an [`UnbindResp`](Pdu::UnbindResp) command to the server.
    pub async fn unbind_resp(self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request(Pdu::UnbindResp, sequence_number)
            .await
    }

    /// Sends an [`EnquireLinkResp`](Pdu::EnquireLinkResp) command to the server.
    pub async fn enquire_link_resp(self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request(Pdu::EnquireLinkResp, sequence_number)
            .await
    }

    /// Sends a [`GenericNack`](Pdu::GenericNack) command to the server.
    pub async fn generic_nack(self, sequence_number: u32) -> Result<(), Error> {
        self.unregistered_request(Pdu::GenericNack, sequence_number)
            .await
    }

    /// Sends a [`BindTransmitter`] command to the server and waits for a successful [`BindTransmitterResp`].
    pub async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<BindTransmitterResp, Error> {
        self.registered_request().bind_transmitter(bind).await
    }

    /// Sends a [`BindReceiver`] command to the server and waits for a successful [`BindReceiverResp`].
    pub async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<BindReceiverResp, Error> {
        self.registered_request().bind_receiver(bind).await
    }

    /// Sends a [`BindTransceiver`] command to the server and waits for a successful [`BindTransceiverResp`].
    pub async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<BindTransceiverResp, Error> {
        self.registered_request().bind_transceiver(bind).await
    }

    /// Sends a [`BroadcastSm`] command to the server and waits for a successful [`BroadcastSmResp`].
    pub async fn broadcast_sm(
        &self,
        broadcast_sm: impl Into<BroadcastSm>,
    ) -> Result<BroadcastSmResp, Error> {
        self.registered_request().broadcast_sm(broadcast_sm).await
    }

    /// Sends a [`CancelBroadcastSm`] command to the server and waits for a successful [`CancelBroadcastSmResp`](Pdu::CancelBroadcastSmResp).
    pub async fn cancel_broadcast_sm(
        &self,
        cancel_broadcast_sm: impl Into<CancelBroadcastSm>,
    ) -> Result<(), Error> {
        self.registered_request()
            .cancel_broadcast_sm(cancel_broadcast_sm)
            .await
    }

    /// Sends a [`CancelSm`] command to the server and waits for a successful [`CancelSmResp`](Pdu::CancelSmResp).
    pub async fn cancel_sm(&self, cancel_sm: impl Into<CancelSm>) -> Result<(), Error> {
        self.registered_request().cancel_sm(cancel_sm).await
    }

    /// Sends a [`DataSm`] command to the server and waits for a successful [`DataSmResp`].
    pub async fn data_sm(&self, data_sm: impl Into<DataSm>) -> Result<DataSmResp, Error> {
        self.registered_request().data_sm(data_sm).await
    }

    /// Sends a [`QueryBroadcastSm`] command to the server and waits for a successful [`QueryBroadcastSmResp`].
    pub async fn query_broadcast_sm(
        &self,
        query_broadcast_sm: impl Into<QueryBroadcastSm>,
    ) -> Result<QueryBroadcastSmResp, Error> {
        self.registered_request()
            .query_broadcast_sm(query_broadcast_sm)
            .await
    }

    /// Sends a [`QuerySm`] command to the server and waits for a successful [`QuerySmResp`].
    pub async fn query_sm(&self, query_sm: impl Into<QuerySm>) -> Result<QuerySmResp, Error> {
        self.registered_request().query_sm(query_sm).await
    }

    /// Sends a [`ReplaceSm`] command to the server and waits for a successful [`ReplaceSmResp`](Pdu::ReplaceSmResp).
    pub async fn replace_sm(&self, replace_sm: impl Into<ReplaceSm>) -> Result<(), Error> {
        self.registered_request().replace_sm(replace_sm).await
    }

    /// Sends a [`SubmitMulti`] command to the server and waits for a successful [`SubmitMultiResp`].
    pub async fn submit_multi(
        &self,
        submit_multi: impl Into<SubmitMulti>,
    ) -> Result<SubmitMultiResp, Error> {
        self.registered_request().submit_multi(submit_multi).await
    }

    /// Sends a [`SubmitSm`] command to the server and waits for a successful [`SubmitSmResp`].
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
        self.registered_request().submit_sm(submit_sm).await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server and waits for a successful [`UnbindResp`](Pdu::UnbindResp).
    pub async fn unbind(&self) -> Result<(), Error> {
        self.registered_request().unbind().await
    }

    /// Sends an [`EnquireLink`](Pdu::EnquireLink) command to the server and waits for a successful [`EnquireLinkResp`](Pdu::EnquireLinkResp).
    pub async fn enquire_link(&self) -> Result<(), Error> {
        self.registered_request().enquire_link().await
    }
}

/// Builder for creating a registered requests.
///
/// Registered requests are requests that expect a response from the server, such as [`BindTransmitter`](Pdu::BindTransmitter), [`SubmitSm`](Pdu::SubmitSm), and [`QuerySm`](Pdu::QuerySm).
#[derive(Debug)]
pub struct RegisteredRequestBuilder<'a, T> {
    client: &'a Client<T>,
    status: CommandStatus,
    response_timeout: Option<Duration>,
}

/// Extracts a specific [`Pdu`] from a generic [`Pdu`].
macro_rules! extract {
    ($pdu:ident) => {
        |pdu| match pdu {
            Pdu::$pdu(response) => Ok(response),
            _ => Err(pdu),
        }
    };
}

impl<'a, T: Timeout> RegisteredRequestBuilder<'a, T> {
    fn new(client: &'a Client<T>, status: CommandStatus) -> Self {
        Self {
            client,
            status,
            response_timeout: client.inner.response_timeout,
        }
    }

    /// Sets the command status for the next request.
    pub const fn status(mut self, status: CommandStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the response timeout for the next request.
    pub fn response_timeout(mut self, timeout: Duration) -> Self {
        self.response_timeout = Some(timeout);
        self
    }

    /// Disables the response timeout for the next request.
    pub fn no_response_timeout(mut self) -> Self {
        self.response_timeout = None;
        self
    }

    fn check_interface_version(&self, interface_version: InterfaceVersion) -> Result<(), Error> {
        if self.client.inner.check_interface_version
            && !matches!(interface_version, InterfaceVersion::Smpp5_0)
        {
            return Err(Error::unsupported_interface_version(interface_version));
        }

        Ok(())
    }

    fn request(&self, pdu: impl Into<Pdu>) -> impl Future<Output = Result<Command, Error>> {
        let pdu = pdu.into();

        async move {
            let sequence_number = self.client.inner.next_sequence_number();

            let command = Command::builder()
                .status(self.status)
                .sequence_number(sequence_number)
                .pdu(pdu.clone());

            let (mut outcome, cell, id) = self.client.inner.send_registered(command).await?;

            tracing::trace!(target: TARGET, sequence_number, timeout = ?self.response_timeout, "Waiting for the write acknowledgement and the response");

            let exchange = self.client.inner.await_terminal_with_timeout(
                &mut outcome,
                &cell,
                id,
                self.response_timeout,
            );

            RequestFutureGuard::new(&self.client.inner.actions, id, cell.clone(), exchange).await
        }
    }

    async fn request_extract<R>(
        &self,
        pdu: impl Into<Pdu>,
        extract: fn(Pdu) -> Result<R, Pdu>,
    ) -> Result<R, Error> {
        self.request(pdu.into())
            .await?
            .ok()
            .map_err(Error::unexpected_response)
            .map(Command::into_parts)
            .map(CommandParts::raw)
            .map(|(id, status, sequence_number, pdu)| {
                pdu.ok_or(CommandParts::new(id, status, sequence_number, None))
                    .and_then(|pdu| {
                        extract(pdu).map_err(|pdu| {
                            CommandParts::new(id, status, sequence_number, Some(pdu))
                        })
                    })
                    .map_err(Command::from_parts)
            })?
            .map_err(Error::unexpected_response)
    }

    /// Sends a [`Pdu`] to the server and waits for a successful response matching the given [`CommandId`].
    async fn request_ok_and_matches(
        &self,
        pdu: impl Into<Pdu>,
        id: CommandId,
    ) -> Result<(), Error> {
        self.request(pdu.into())
            .await?
            .ok_and_matches(id)
            .map(|_| ())
            .map_err(Error::unexpected_response)
    }

    /// Sends a [`BindTransmitter`] command to the server and waits for a successful [`BindTransmitterResp`].
    pub async fn bind_transmitter(
        &self,
        bind: impl Into<BindTransmitter>,
    ) -> Result<BindTransmitterResp, Error> {
        let bind: BindTransmitter = bind.into();

        self.check_interface_version(bind.interface_version)?;

        self.request_extract(bind, extract!(BindTransmitterResp))
            .await
    }

    /// Sends a [`BindReceiver`] command to the server and waits for a successful [`BindReceiverResp`].
    pub async fn bind_receiver(
        &self,
        bind: impl Into<BindReceiver>,
    ) -> Result<BindReceiverResp, Error> {
        let bind: BindReceiver = bind.into();

        self.check_interface_version(bind.interface_version)?;

        self.request_extract(bind, extract!(BindReceiverResp)).await
    }

    /// Sends a [`BindTransceiver`] command to the server and waits for a successful [`BindTransceiverResp`].
    pub async fn bind_transceiver(
        &self,
        bind: impl Into<BindTransceiver>,
    ) -> Result<BindTransceiverResp, Error> {
        let bind: BindTransceiver = bind.into();

        self.check_interface_version(bind.interface_version)?;

        self.request_extract(bind, extract!(BindTransceiverResp))
            .await
    }

    /// Sends a [`BroadcastSm`] command to the server and waits for a successful [`BroadcastSmResp`].
    pub async fn broadcast_sm(
        &self,
        broadcast_sm: impl Into<BroadcastSm>,
    ) -> Result<BroadcastSmResp, Error> {
        self.request_extract(broadcast_sm.into(), extract!(BroadcastSmResp))
            .await
    }

    /// Sends a [`CancelBroadcastSm`] command to the server and waits for a successful [`CancelBroadcastSmResp`](Pdu::CancelBroadcastSmResp).
    pub async fn cancel_broadcast_sm(
        &self,
        cancel_broadcast_sm: impl Into<CancelBroadcastSm>,
    ) -> Result<(), Error> {
        self.request_ok_and_matches(cancel_broadcast_sm.into(), CommandId::CancelBroadcastSmResp)
            .await
    }

    /// Sends a [`CancelSm`] command to the server and waits for a successful [`CancelSmResp`](Pdu::CancelSmResp).
    pub async fn cancel_sm(&self, cancel_sm: impl Into<CancelSm>) -> Result<(), Error> {
        self.request_ok_and_matches(cancel_sm.into(), CommandId::CancelSmResp)
            .await
    }

    /// Sends a [`DataSm`] command to the server and waits for a successful [`DataSmResp`].
    pub async fn data_sm(&self, data_sm: impl Into<DataSm>) -> Result<DataSmResp, Error> {
        self.request_extract(data_sm.into(), extract!(DataSmResp))
            .await
    }

    /// Sends a [`QueryBroadcastSm`] command to the server and waits for a successful [`QueryBroadcastSmResp`].
    pub async fn query_broadcast_sm(
        &self,
        query_broadcast_sm: impl Into<QueryBroadcastSm>,
    ) -> Result<QueryBroadcastSmResp, Error> {
        self.request_extract(query_broadcast_sm.into(), extract!(QueryBroadcastSmResp))
            .await
    }

    /// Sends a [`QuerySm`] command to the server and waits for a successful [`QuerySmResp`].
    pub async fn query_sm(&self, query_sm: impl Into<QuerySm>) -> Result<QuerySmResp, Error> {
        self.request_extract(query_sm.into(), extract!(QuerySmResp))
            .await
    }

    /// Sends a [`ReplaceSm`] command to the server and waits for a successful [`ReplaceSmResp`](Pdu::ReplaceSmResp).
    pub async fn replace_sm(&self, replace_sm: impl Into<ReplaceSm>) -> Result<(), Error> {
        self.request_ok_and_matches(replace_sm.into(), CommandId::ReplaceSmResp)
            .await
    }

    /// Sends a [`SubmitMulti`] command to the server and waits for a successful [`SubmitMultiResp`].
    pub async fn submit_multi(
        &self,
        submit_multi: impl Into<SubmitMulti>,
    ) -> Result<SubmitMultiResp, Error> {
        self.request_extract(submit_multi.into(), extract!(SubmitMultiResp))
            .await
    }

    /// Sends a [`SubmitSm`] command to the server and waits for a successful [`SubmitSmResp`].
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<SubmitSmResp, Error> {
        self.request_extract(submit_sm.into(), extract!(SubmitSmResp))
            .await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server and waits for a successful [`UnbindResp`](Pdu::UnbindResp).
    pub async fn unbind(&self) -> Result<(), Error> {
        self.request_ok_and_matches(Pdu::Unbind, CommandId::UnbindResp)
            .await
    }

    /// Sends an [`EnquireLink`](Pdu::EnquireLink) command to the server and waits for a successful [`EnquireLinkResp`](Pdu::EnquireLinkResp).
    pub async fn enquire_link(&self) -> Result<(), Error> {
        self.request_ok_and_matches(Pdu::EnquireLink, CommandId::EnquireLinkResp)
            .await
    }
}

/// Builder for creating a no-wait requests.
#[derive(Debug)]
pub struct NoWaitRequestBuilder<'a, T> {
    client: &'a Client<T>,
    status: CommandStatus,
}

impl<'a, T: Timeout> NoWaitRequestBuilder<'a, T> {
    const fn new(client: &'a Client<T>, status: CommandStatus) -> Self {
        Self { client, status }
    }

    /// Sets the command status for the next request.
    pub const fn status(mut self, status: CommandStatus) -> Self {
        self.status = status;
        self
    }

    /// Sends a [`Pdu`] to the server without waiting for the response.
    async fn send(&self, pdu: impl Into<Pdu>) -> Result<u32, Error> {
        let sequence_number = self.client.inner.next_sequence_number();
        let request_id = self.client.inner.next_request_id();

        let command = Command::builder()
            .status(self.status)
            .sequence_number(sequence_number)
            .pdu(pdu.into());

        let (request, mut outcome) = UnregisteredRequest::new(request_id, command);
        let cell = request.cell.clone();

        if self
            .client
            .inner
            .actions
            .send(Action::unregistered_request(request))
            .is_err()
        {
            // The connection's action channel is gone: nothing was queued and nothing was
            // written, so this is a definite "not sent", not a closed-connection outcome.
            return Err(Error::not_sent(NotSentReason::ConnectionClosed));
        }

        // The write stage is guarded: dropping this future while the request is queued must
        // abandon it — a send the caller gave up on must not reach the peer.
        let write_stage = async {
            match outcome.recv().await {
                Some(Outcome::Written { sequence_number }) => Ok(sequence_number),
                // Unreachable: no response is ever routed to an unregistered request.
                Some(Outcome::ResponseReady) => Ok(sequence_number),
                Some(Outcome::Failed(error)) => Err(error),
                None => Err(closed_without_outcome(&cell)),
            }
        };

        RequestFutureGuard::new(
            &self.client.inner.actions,
            request_id,
            cell.clone(),
            write_stage,
        )
        .await
    }

    /// Sends a [`BroadcastSm`] command to the server without waiting for the response.
    pub async fn broadcast_sm(&self, broadcast_sm: impl Into<BroadcastSm>) -> Result<u32, Error> {
        self.send(broadcast_sm.into()).await
    }

    /// Sends a [`CancelBroadcastSm`] command to the server without waiting for the response.
    pub async fn cancel_broadcast_sm(
        &self,
        cancel_broadcast_sm: impl Into<CancelBroadcastSm>,
    ) -> Result<u32, Error> {
        self.send(cancel_broadcast_sm.into()).await
    }

    /// Sends a [`CancelSm`] command to the server without waiting for the response.
    pub async fn cancel_sm(&self, cancel_sm: impl Into<CancelSm>) -> Result<u32, Error> {
        self.send(cancel_sm.into()).await
    }

    /// Sends a [`DataSm`] command to the server without waiting for the response.
    pub async fn data_sm(&self, data_sm: impl Into<DataSm>) -> Result<u32, Error> {
        self.send(data_sm.into()).await
    }

    /// Sends a [`QueryBroadcastSm`] command to the server without waiting for the response.
    pub async fn query_broadcast_sm(
        &self,
        query_broadcast_sm: impl Into<QueryBroadcastSm>,
    ) -> Result<u32, Error> {
        self.send(query_broadcast_sm.into()).await
    }

    /// Sends a [`QuerySm`] command to the server without waiting for the response.
    pub async fn query_sm(&self, query_sm: impl Into<QuerySm>) -> Result<u32, Error> {
        self.send(query_sm.into()).await
    }

    /// Sends a [`ReplaceSm`] command to the server without waiting for the response.
    pub async fn replace_sm(&self, replace_sm: impl Into<ReplaceSm>) -> Result<u32, Error> {
        self.send(replace_sm.into()).await
    }

    /// Sends a [`SubmitMulti`] command to the server without waiting for the response.
    pub async fn submit_multi(&self, submit_multi: impl Into<SubmitMulti>) -> Result<u32, Error> {
        self.send(submit_multi.into()).await
    }

    /// Sends a [`SubmitSm`] command to the server without waiting for the response.
    pub async fn submit_sm(&self, submit_sm: impl Into<SubmitSm>) -> Result<u32, Error> {
        self.send(submit_sm.into()).await
    }

    /// Sends an [`Unbind`](Pdu::Unbind) command to the server without waiting for the response.
    pub async fn unbind(&self) -> Result<u32, Error> {
        self.send(Pdu::Unbind).await
    }

    /// Sends an [`EnquireLink`](Pdu::EnquireLink) command to the server without waiting for the response.
    pub async fn enquire_link(&self) -> Result<u32, Error> {
        self.send(Pdu::EnquireLink).await
    }
}

/// Builder for creating a raw registered requests.
///
/// Raw registered requests are requests that expect a response from the server, but do not poll the response automatically.
#[derive(Debug)]
pub struct RawRegisteredRequestBuilder<'a, T> {
    client: &'a Client<T>,
    status: CommandStatus,
    response_timeout: Option<Duration>,
}

impl<'a, T: Timeout> RawRegisteredRequestBuilder<'a, T> {
    fn new(client: &'a Client<T>, status: CommandStatus) -> Self {
        Self {
            client,
            status,
            response_timeout: client.inner.response_timeout,
        }
    }

    /// Sets the command status for the next request.
    pub const fn status(mut self, status: CommandStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the response timeout for the next request.
    pub fn response_timeout(mut self, timeout: Duration) -> Self {
        self.response_timeout = Some(timeout);
        self
    }

    /// Disables the response timeout for the next request.
    pub fn no_response_timeout(mut self) -> Self {
        self.response_timeout = None;
        self
    }

    /// Sends a raw [`Pdu`] to the server and returns the sent `sequence number` and a future resolving to a successful response [`Command`].
    ///
    /// # Notes
    ///
    /// - If the sent command is not an operation expecting a response, and the response timeout is unset, the response future will never resolve and should be dropped.
    /// - The response timeout is started when the response future is awaited.
    /// - No interface version check is performed.
    pub fn send(
        self,
        pdu: impl Into<Pdu>,
    ) -> impl Future<Output = Result<(u32, impl Future<Output = Result<Command, Error>>), Error>>
    {
        let pdu = pdu.into();

        async move {
            let sequence_number = self.client.inner.next_sequence_number();

            let command = Command::builder()
                .status(self.status)
                .sequence_number(sequence_number)
                .pdu(pdu.clone());

            let id = command.id();

            let request_id = self.client.inner.next_request_id();
            let (request, mut outcome) = RegisteredRequest::new(request_id, command);
            let cell = request.cell.clone();

            if self
                .client
                .inner
                .actions
                .send(Action::registered_request(request))
                .is_err()
            {
                // Nothing was queued and nothing was written: a definite "not sent".
                return Err(Error::not_sent(NotSentReason::ConnectionClosed));
            }

            // The write stage: it resolves once the request is with the transport, or
            // earlier when the response arrives first — the peer has confirmed the request
            // either way, and a response already decoded is never discarded by a later
            // failure or teardown.
            //
            // The response itself stays in the request's cell across the two stages: the
            // stage only marks it as available (its sequence number), and the response
            // future consumes it from the cell — so a caller that drops the returned
            // future routes the unconsumed response late through the cell's own drop path
            // instead of losing it with the future.
            let write_stage = async {
                loop {
                    match outcome.recv().await {
                        Some(Outcome::Written { sequence_number }) => {
                            return Ok((sequence_number, None));
                        }
                        Some(Outcome::ResponseReady) => {
                            if let Some(sequence_number) = cell.stored_response_sequence_number() {
                                return Ok((sequence_number, Some(sequence_number)));
                            }
                            // Defensive: the notification and the stored response are
                            // written together, under the cell's lock; nothing else can
                            // take it before this stage resolves.
                        }
                        Some(Outcome::Failed(error)) => return Err(error),
                        None => return Err(closed_without_outcome(&cell)),
                    }
                }
            };

            let (sequence_number, confirmed) = RequestFutureGuard::new(
                &self.client.inner.actions,
                request_id,
                cell.clone(),
                write_stage,
            )
            .await?;

            let timeout_cell = cell.clone();
            let guard_cell = cell.clone();
            let response_timeout = self.response_timeout;

            let response = async move {
                let command = match confirmed {
                    // The write stage saw the response stored in the cell: consume it from
                    // there. (Nothing else can have taken it: the timeout arm below runs
                    // only after this stage, and the cell's drop path only when the whole
                    // future is dropped, which skips this code entirely.)
                    Some(_) => match cell.take_response() {
                        Some(command) => command,
                        None => return Err(closed_without_outcome(&cell)),
                    },
                    None => await_terminal(&mut outcome, &cell).await?,
                };

                // XXX: it is ok to match against responses only, as this is a registered request
                // If the request does not have a matching response, the user should not be awaiting it here anyway
                command
                    .ok_and_matches(id.matching_response())
                    .map_err(Error::unexpected_response)
            };

            let future = async move {
                match response_timeout {
                    None => response.await,
                    Some(timeout) => match T::timeout(timeout, response).await {
                        Some(result) => result,
                        None => {
                            // The atomic settlement: a response committed to the cell
                            // before this decision wins — the caller gets it, never a
                            // timeout for a request the peer already answered. Only when
                            // nothing is stored does the timeout abandon the request, and
                            // every later commit goes to the late path instead.
                            match timeout_cell.settle_timeout() {
                                // A response committed before the settlement wins. It
                                // takes the same validation as the response stage: a
                                // command the server sent under our sequence number is
                                // not automatically this request's answer.
                                TimeoutSettlement::Response(command) => command
                                    .ok_and_matches(id.matching_response())
                                    .map_err(Error::unexpected_response),
                                TimeoutSettlement::Abandoned(abandoned) => {
                                    let _ =
                                        self.client.inner.actions.send(Action::Cancel(request_id));

                                    Err(match abandoned {
                                        AbandonOutcome::Written { sequence_number } => {
                                            Error::response_timeout(sequence_number, timeout)
                                        }
                                        // Unreachable: the write stage resolved with the
                                        // acknowledgement, so the gate had already claimed
                                        // the request.
                                        AbandonOutcome::NotWritten => {
                                            Error::not_sent(NotSentReason::Timeout)
                                        }
                                    })
                                }
                            }
                        }
                    },
                }
            };

            Ok((
                sequence_number,
                RequestFutureGuard::new(&self.client.inner.actions, request_id, guard_cell, future),
            ))
        }
    }
}
