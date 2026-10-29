use std::{
    collections::BTreeMap,
    pin::{Pin, pin},
    time::Duration,
};

use futures::{
    Sink, SinkExt, Stream, StreamExt,
    future::{Either, select},
};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    tokio_codec::{DecodeError, EncodeError},
};
use tokio::sync::{
    mpsc::{Sender, UnboundedSender, channel, unbounded_channel},
    oneshot, watch,
};
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};

use crate::{
    Action, FairStream, SequenceNumber, Timer, UrgentAction, error::Error, event::Insight,
    event_::EventChannel, runtime_::Delay,
};

const CONNECTION_SMPP: &str = "rusmppc::connection::smpp";
const CONNECTION_TCP: &str = "rusmppc::connection::tcp";
const TIMER: &str = "rusmppc::connection::smpp::timer";

pub struct Connection<F, E, D: Delay> {
    sequence_number: SequenceNumber,
    framed: F,
    urgent_actions: UnboundedReceiverStream<UrgentAction>,
    actions: ReceiverStream<Action>,
    events: E,
    responses: BTreeMap<u32, Response>,
    enquire_link_interval: Option<Duration>,
    enquire_link_response_timeout: Duration,
    enquire_link_timer: Timer<D>,
    enquire_link_response_timer: Timer<D>,
    auto_enquire_link_response: bool,
    /// Used to let the [`Client`](crate::Client) wait for the [`Connection`] to be closed
    _watch: watch::Receiver<()>,
}

impl<F, E: EventChannel, D: Delay> Connection<F, E, D> {
    #[allow(clippy::type_complexity)]
    pub fn new(
        sequence_number: SequenceNumber,
        framed: F,
        enquire_link_interval: Option<Duration>,
        enquire_link_response_timeout: Duration,
        auto_enquire_link_response: bool,
    ) -> (
        Self,
        watch::Sender<()>,
        UnboundedSender<UrgentAction>,
        Sender<Action>,
        UnboundedReceiverStream<E::Event>,
    ) {
        // TODO: make this bound
        let (events_tx, events_rx) = unbounded_channel::<E::Event>();
        let events = E::new(events_tx);

        let (urgent_actions_tx, urgent_actions_rx) = unbounded_channel::<UrgentAction>();
        let (actions_tx, actions_rx) = channel::<Action>(100);
        let (watch_tx, watch_rx) = watch::channel(());

        let enquire_link_timer = enquire_link_interval
            .map(|duration| Timer::<D>::active(duration))
            .unwrap_or(Timer::<D>::inactive());

        let enquire_link_response_timer = Timer::<D>::inactive();

        (
            Self {
                sequence_number,
                framed,
                urgent_actions: UnboundedReceiverStream::new(urgent_actions_rx),
                actions: ReceiverStream::new(actions_rx),
                events,
                responses: BTreeMap::new(),
                enquire_link_interval,
                enquire_link_response_timeout,
                enquire_link_timer,
                enquire_link_response_timer,
                auto_enquire_link_response,
                _watch: watch_rx,
            },
            watch_tx,
            urgent_actions_tx,
            actions_tx,
            UnboundedReceiverStream::new(events_rx),
        )
    }

    pub fn with_framed<Fr>(self, framed: Fr) -> Connection<Fr, E, D> {
        Connection {
            sequence_number: self.sequence_number,
            framed,
            urgent_actions: self.urgent_actions,
            actions: self.actions,
            events: self.events,
            responses: self.responses,
            enquire_link_interval: self.enquire_link_interval,
            enquire_link_response_timeout: self.enquire_link_response_timeout,
            enquire_link_timer: self.enquire_link_timer,
            enquire_link_response_timer: self.enquire_link_response_timer,
            auto_enquire_link_response: self.auto_enquire_link_response,
            _watch: self._watch,
        }
    }
}

impl<F, E: EventChannel, D: Delay> Connection<F, E, D>
where
    F: Stream<Item = Result<Command, DecodeError>> + for<'a> Sink<&'a Command, Error = EncodeError>,
{
    #[allow(clippy::collapsible_if)]
    pub async fn run(mut self) {
        let mut actions = pin!(FairStream::new(self.actions, 5));
        let mut framed = pin!(FairStream::new(self.framed, 5));
        let mut enquire_link_timer = pin!(self.enquire_link_timer);
        let mut enquire_link_response_timer = pin!(self.enquire_link_response_timer);

        loop {
            tracing::trace!(target: CONNECTION_SMPP, "Polling for events");

            match ready(
                enquire_link_timer.as_mut(),
                enquire_link_response_timer.as_mut(),
                &mut self.urgent_actions,
                actions.as_mut(),
                framed.as_mut(),
            )
            .await
            {
                Ready::EnquireLinkTimer => {
                    tracing::trace!(target: TIMER, "Enquire link timer fired");

                    let sequence_number = self.sequence_number.next();

                    let command = Command::builder()
                        .status(CommandStatus::EsmeRok)
                        .sequence_number(sequence_number)
                        .pdu(Pdu::EnquireLink);

                    tracing::debug!(target: CONNECTION_SMPP, sequence_number, "Sending enquire link");

                    if let Err(err) = framed.send(&command).await {
                        tracing::error!(target: CONNECTION_SMPP, sequence_number, ?err, "Error sending enquire link");

                        let _ = self.events.send_error(Error::from(err));

                        break;
                    }

                    self.responses.insert(
                        sequence_number,
                        Response::Connection(ConnectionCommandId::EnquireLink),
                    );

                    enquire_link_response_timer
                        .as_mut()
                        .activate(self.enquire_link_response_timeout);

                    enquire_link_timer.as_mut().deactivate();

                    let _ = self
                        .events
                        .send_insight(Insight::SentEnquireLink(sequence_number));
                }
                Ready::EnquireLinkResponseTimer => {
                    tracing::trace!(target: TIMER, "Enquire link response timer fired");

                    let timeout = self.enquire_link_response_timeout;

                    let _ = self
                        .events
                        .send_error(Error::EnquireLinkTimeout { timeout });

                    break;
                }
                Ready::UrgentAction(action) => {
                    let action = match action {
                        Some(action) => action,
                        None => {
                            tracing::debug!(target: CONNECTION_SMPP, "Urgent action channel closed");

                            break;
                        }
                    };

                    match action {
                        UrgentAction::RemoveResponse(sequence_number) => {
                            tracing::debug!(target: CONNECTION_SMPP, sequence_number, "Received remove response");

                            self.responses.remove(&sequence_number);
                        }
                        UrgentAction::Close(close) => {
                            tracing::debug!(target: CONNECTION_SMPP, "Received close request");

                            self.urgent_actions.close();

                            actions.inner_mut().close();

                            let _ = close.ack.send(());

                            break;
                        }
                    }
                }
                Ready::Action(action) => {
                    let action = match action {
                        Some(action) => action,
                        None => {
                            tracing::debug!(target: CONNECTION_SMPP, "Action channel closed");

                            break;
                        }
                    };

                    match action {
                        Action::Request(request) => {
                            let sequence_number = request.command.sequence_number();
                            let status = request.command.status();
                            let id = request.command.id();

                            tracing::debug!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Received request");

                            if request.start_send.send(()).is_err() {
                                tracing::debug!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Request was dropped. Ignoring request");

                                continue;
                            }

                            tracing::debug!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Sending command");

                            match framed.send(&request.command).await {
                                Ok(_) => {
                                    tracing::debug!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Sent command");

                                    if let Some(response) = request.response {
                                        self.responses
                                            .insert(sequence_number, Response::Client(response));

                                        tracing::debug!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Registered response");
                                    }

                                    let _ = request.sent.send(Ok(()));
                                }
                                Err(err) => {
                                    tracing::error!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, ?err, "Error sending command");

                                    if let Err(Err(err)) = request.sent.send(Err(Error::from(err)))
                                    {
                                        tracing::trace!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Client not waiting");

                                        let _ = self.events.send_error(err);
                                    }

                                    break;
                                }
                            }
                        }
                        Action::PendingResponses(responses) => {
                            let pending = self.responses.keys().copied().collect();

                            let _ = responses.ack.send(Ok(pending));
                        }
                    }
                }
                Ready::Framed(command) => {
                    let command = match command {
                        Some(Ok(command)) => command,
                        Some(Err(err)) => {
                            tracing::error!(target: CONNECTION_SMPP, ?err, "Error reading from stream");

                            let _ = self.events.send_error(Error::from(err));

                            break;
                        }
                        None => {
                            let _ = self.events.send_error(Error::UnexpectedEndOfStream);

                            break;
                        }
                    };

                    let sequence_number = command.sequence_number();
                    let status = command.status();
                    let id = command.id();

                    tracing::debug!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Received command");

                    // reset the enquire link timer since the connection is active
                    if let Some(delay) = self.enquire_link_interval {
                        tracing::trace!(target: TIMER, "Resetting enquire link timer");

                        enquire_link_timer.as_mut().reactivate(delay);
                    }

                    if let CommandId::EnquireLink = command.id()
                        && self.auto_enquire_link_response
                    {
                        let _ = self
                            .events
                            .send_insight(Insight::ReceivedEnquireLink(sequence_number));

                        let response = Command::builder()
                            .status(CommandStatus::EsmeRok)
                            .sequence_number(sequence_number)
                            .pdu(Pdu::EnquireLinkResp);

                        tracing::debug!(target: CONNECTION_SMPP, sequence_number, "Sending enquire link response");

                        if let Err(err) = framed.send(&response).await {
                            tracing::error!(target: CONNECTION_SMPP, sequence_number, ?err, "Error sending enquire link response");

                            let _ = self.events.send_error(Error::from(err));

                            break;
                        }

                        let _ = self
                            .events
                            .send_insight(Insight::SentEnquireLinkResp(sequence_number));

                        continue;
                    }

                    if id.is_response() {
                        tracing::trace!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Command is a response");

                        if let Some(response) = self.responses.remove(&sequence_number) {
                            tracing::trace!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Found matching response");

                            match response {
                                Response::Client(sender) => {
                                    if let Err(command) = sender.send(command) {
                                        tracing::trace!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Client not waiting");

                                        let _ = self.events.send_incoming(command);
                                    }
                                }
                                Response::Connection(ConnectionCommandId::EnquireLink) => {
                                    if let CommandId::EnquireLinkResp = id
                                        && let CommandStatus::EsmeRok = status
                                    {
                                        enquire_link_response_timer.as_mut().deactivate();

                                        if let Some(delay) = self.enquire_link_interval {
                                            enquire_link_timer.as_mut().activate(delay);
                                        }

                                        let _ = self.events.send_insight(
                                            Insight::ReceivedEnquireLinkResp(sequence_number),
                                        );
                                    }
                                }
                            }
                        } else {
                            tracing::trace!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "No matching response found");

                            let _ = self.events.send_incoming(command);
                        }
                    } else {
                        tracing::trace!(target: CONNECTION_SMPP, sequence_number, ?status, ?id, "Command is a request");

                        let _ = self.events.send_incoming(command);
                    }
                }
            };
        }

        tracing::debug!(target: CONNECTION_TCP, "Shutting down stream");

        if let Err(err) = framed.close().await {
            tracing::error!(target: CONNECTION_TCP, ?err, "Failed to shutdown stream");
        }

        tracing::debug!(target: CONNECTION_SMPP, "Connection closed");
    }
}

enum Ready {
    EnquireLinkTimer,
    EnquireLinkResponseTimer,
    UrgentAction(Option<UrgentAction>),
    Action(Option<Action>),
    Framed(Option<Result<Command, DecodeError>>),
}

enum Response {
    /// A response that should be sent to the client
    Client(oneshot::Sender<Command>),
    /// A response for a command that was sent by the [`Connection`].
    Connection(ConnectionCommandId),
}

/// Command IDs issued by the [`Connection`] itself.
enum ConnectionCommandId {
    EnquireLink,
}

async fn ready<F, U, A, D>(
    mut enquire_link_timer: Pin<&mut Timer<D>>,
    mut enquire_link_response_timer: Pin<&mut Timer<D>>,
    urgent_actions: &mut U,
    mut actions: Pin<&mut A>,
    mut framed: Pin<&mut F>,
) -> Ready
where
    F: Stream<Item = Result<Command, DecodeError>> + for<'a> Sink<&'a Command, Error = EncodeError>,
    U: Stream<Item = UrgentAction> + Unpin,
    A: Stream<Item = Action> + Unpin,
    D: Delay,
{
    match select(
        select(
            select(&mut enquire_link_timer, &mut enquire_link_response_timer),
            select(urgent_actions.next(), actions.next()),
        ),
        framed.next(),
    )
    .await
    {
        Either::Left((left, _)) => match left {
            Either::Left((left, _)) => match left {
                Either::Left((_, _)) => Ready::EnquireLinkTimer,
                Either::Right((_, _)) => Ready::EnquireLinkResponseTimer,
            },
            Either::Right((right, _)) => match right {
                Either::Left((action, _)) => Ready::UrgentAction(action),
                Either::Right((action, _)) => Ready::Action(action),
            },
        },
        Either::Right((command, _)) => Ready::Framed(command),
    }
}

// TODO: enable
// #[cfg(test)]
// mod tests;
