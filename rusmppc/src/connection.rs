use std::{
    collections::{BTreeMap, VecDeque},
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use crate::{
    Action, Client, Request, Timer,
    builder::NoSpawnConnectionBuilder,
    delay::Delay,
    error::Error,
    event::{EventChannel, Insight},
    request::ObligatedRequest,
};
use futures::{FutureExt, Sink, SinkExt, Stream};
use pin_project_lite::pin_project;
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    tokio_codec::{DecodeError, EncodeError},
};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot, watch,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

const CONN: &str = "rusmppc::connection::smpp";
const TIMER: &str = "rusmppc::connection::smpp::timer";

const ACTIONS_POLL_LIMIT: u8 = 5;
const SINK_POLL_LIMIT: u8 = 5;
const STREAM_POLL_LIMIT: u8 = 5;

#[derive(Debug)]
enum State {
    Active,
    /// The user sent a close request.
    Closing,
    Errored,
}

// Make sure to drop the Connection after completion to prevent clients from queueing more actions.
// This way if the Connection was closed and the Connection is not in an active state but not dropped,
// clients will still be able to send actions and will not get an immediate error that the channel is closed.
// Actions will not be queued and the client would wait forever until the Connection is dropped.
// We rely on this mechanism to work, to report correct and predictable errors.
pin_project! {
    pub struct Connection<F, D1: Delay, D2: Delay, E> {
        state: State,
        sequence_number: u32,
        requests: VecDeque<Request>,
        // This is a request that has been written to the sink using start_send, but not yet flushed.
        pending_request: Option<Request>,
        responses: BTreeMap<u32, oneshot::Sender<Command>>,
        enquire_link_interval: Option<Duration>,
        last_enquire_link_sequence_number: Option<u32>,
        enquire_link_response_timeout: Duration,
        auto_enquire_link_response: bool,
        events: E,
        // Used to let the client wait for the connection to be closed
        _watch: watch::Receiver<()>,
        #[pin]
        enquire_link_timer: Timer<D1>,
        #[pin]
        enquire_link_response_timer: Timer<D2>,
        #[pin]
        framed: F,
        #[pin]
        actions: UnboundedReceiverStream<Action>,
    }
}

impl<D1: Delay, D2: Delay, E: EventChannel> Connection<(), D1, D2, E> {
    pub fn new(
        enquire_link_interval: Option<Duration>,
        enquire_link_response_timeout: Duration,
        auto_enquire_link_response: bool,
        enquire_link_timer_delay: D1,
        enquire_link_response_timer_delay: D2,
    ) -> (
        Self,
        watch::Sender<()>,
        UnboundedSender<Action>,
        UnboundedReceiverStream<E::Event>,
    ) {
        let (events_tx, events_rx) = mpsc::unbounded_channel::<E::Event>();
        let events = E::new(events_tx);

        let (actions_tx, actions_rx) = mpsc::unbounded_channel::<Action>();
        let (watch_tx, watch_rx) = watch::channel(());

        (
            Self {
                state: State::Active,
                sequence_number: 2,
                requests: VecDeque::new(),
                pending_request: None,
                responses: BTreeMap::new(),
                enquire_link_interval,
                last_enquire_link_sequence_number: None,
                enquire_link_response_timeout,
                auto_enquire_link_response,
                enquire_link_timer: enquire_link_interval
                    .map(|duration| Timer::active(enquire_link_timer_delay, duration))
                    .unwrap_or_default(),
                enquire_link_response_timer: Timer::inactive(enquire_link_response_timer_delay),
                _watch: watch_rx,
                events,
                framed: (),
                actions: UnboundedReceiverStream::new(actions_rx),
            },
            watch_tx,
            actions_tx,
            UnboundedReceiverStream::new(events_rx),
        )
    }

    pub fn with_framed<F>(self, framed: F) -> Connection<F, D1, D2, E> {
        Connection {
            state: self.state,
            sequence_number: self.sequence_number,
            requests: self.requests,
            pending_request: self.pending_request,
            responses: self.responses,
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

impl<F, D1: Delay, D2: Delay, E> Connection<F, D1, D2, E>
where
    F: Stream<Item = Result<Command, DecodeError>> + for<'a> Sink<&'a Command, Error = EncodeError>,
    E: EventChannel,
{
    fn insert_response(
        self: Pin<&mut Self>,
        sequence_number: u32,
        response: oneshot::Sender<Command>,
    ) {
        self.project().responses.insert(sequence_number, response);
    }

    fn remove_response(
        self: Pin<&mut Self>,
        sequence_number: u32,
    ) -> Option<oneshot::Sender<Command>> {
        self.project().responses.remove(&sequence_number)
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

        *(self.project().sequence_number) += 2;

        sequence_number
    }
}

impl<F, D1: Delay, D2: Delay, E> Future for Connection<F, D1, D2, E>
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
                                    self.as_mut().project().responses.keys().copied().collect();

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
                            Action::Remove(sequence_number) => {
                                tracing::debug!(target: CONN, sequence_number, "Received remove response");

                                self.as_mut().remove_response(sequence_number);
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
                                    tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Sent command");

                                    match request {
                                        Request::Registered(request) => {
                                            tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Registered");

                                            let _ = request.ack.send(Ok(()));

                                            self.as_mut()
                                                .insert_response(sequence_number, request.response);
                                        }
                                        Request::Unregistered(request) => {
                                            let _ = request.ack.send(Ok(()));
                                        }
                                        Request::Obligated(_) => {
                                            // No ack for obligated requests
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
                                    tracing::error!(target: CONN, ?err);

                                    self.as_mut().set_state(State::Errored);

                                    match request.send_ack(Err(Error::from(err))) {
                                        Ok(()) => {
                                            return Poll::Ready(());
                                        }
                                        Err(Err(err)) => {
                                            // Client not waiting

                                            let _ = self.as_mut().events.send_error(err);

                                            return Poll::Ready(());
                                        }
                                        Err(Ok(_)) => {
                                            unreachable!()
                                        }
                                    }
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
                        Some(request) => {
                            match Sink::<&Command>::poll_ready(self.as_mut().project().framed, cx) {
                                Poll::Ready(Ok(())) => {
                                    let sequence_number = request.command().sequence_number();
                                    let status = request.command().status();
                                    let id = request.command().id();

                                    tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Writing command");

                                    if let Err(err) =
                                        self.as_mut().project().framed.start_send(request.command())
                                    {
                                        tracing::error!(target: CONN, sequence_number, ?status, ?id, ?err);

                                        self.as_mut().set_state(State::Errored);

                                        match request.send_ack(Err(Error::from(err))) {
                                            Ok(()) => {
                                                return Poll::Ready(());
                                            }
                                            Err(Err(err)) => {
                                                let _ = self.as_mut().events.send_error(err);

                                                return Poll::Ready(());
                                            }
                                            Err(Ok(_)) => {
                                                unreachable!()
                                            }
                                        }
                                    }

                                    // Start send was ok, we encoded the command now we set the request as a pending request.

                                    self.as_mut().set_pending_request(request);

                                    continue 'sink;
                                }
                                Poll::Ready(Err(err)) => {
                                    tracing::error!(target: CONN, ?err);

                                    self.as_mut().set_state(State::Errored);

                                    match request.send_ack(Err(Error::from(err))) {
                                        Ok(()) => {
                                            return Poll::Ready(());
                                        }
                                        Err(Err(err)) => {
                                            // Client not waiting

                                            let _ = self.as_mut().events.send_error(err);

                                            return Poll::Ready(());
                                        }
                                        Err(Ok(_)) => {
                                            unreachable!()
                                        }
                                    }
                                }
                                Poll::Pending => {
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
                                match self.as_mut().remove_response(sequence_number) {
                                    Some(response) => {
                                        tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Found response");

                                        match response.send(command) {
                                            Ok(()) => {
                                                // Sent, do nothing
                                            }
                                            Err(command) => {
                                                // Client not waiting, return the command as an incoming event instead

                                                tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Client not waiting");

                                                let _ = self.as_mut().events.send_incoming(command);
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
                                .send_error(Error::ConnectionClosedByPeer);

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

impl<E: EventChannel> NoSpawnConnectionBuilder<E> {
    /// Consumes the builder and creates a new [`Client`] along with the connection future and event stream (from raw parts).
    pub(crate) fn raw<F, D1, D2>(
        self,
        framed: F,
        enquire_link_timer_delay: D1,
        enquire_link_response_timer_delay: D2,
    ) -> (
        Client,
        impl Stream<Item = E::Event> + Unpin + 'static,
        impl Future<Output = ()>,
    )
    where
        D1: Delay,
        D2: Delay,
        F: Stream<Item = Result<Command, DecodeError>>
            + for<'a> Sink<&'a Command, Error = EncodeError>,
    {
        let (connection, watch, actions, events) = Connection::<_, _, _, E>::new(
            self.builder.enquire_link_interval,
            self.builder.enquire_link_response_timeout,
            self.builder.auto_enquire_link_response,
            enquire_link_timer_delay,
            enquire_link_response_timer_delay,
        );

        let client = Client::new(
            actions,
            self.builder.response_timeout,
            self.builder.check_interface_version,
            watch,
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
