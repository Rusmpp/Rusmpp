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
    Command, CommandStatus, Pdu,
    tokio_codec::{DecodeError, EncodeError},
};
use tokio::sync::{
    mpsc::{Sender, channel, unbounded_channel},
    oneshot,
};
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};

use crate::{SequenceNumber, Timer, error::Error, event_::EventChannel, runtime_::Delay};

struct Connection2<F, E, D: Delay> {
    sequence_number: SequenceNumber,
    framed: F,
    requests: ReceiverStream<Request>,
    events: E,
    responses: BTreeMap<u32, Response>,
    enquire_link_interval: Option<Duration>,
    enquire_link_response_timeout: Duration,
    enquire_link_timer: Timer<D>,
    enquire_link_response_timer: Timer<D>,
    auto_enquire_link_response: bool,
}

impl<F, E: EventChannel, D: Delay> Connection2<F, E, D> {
    fn new(
        sequence_number: SequenceNumber,
        framed: F,
        enquire_link_interval: Option<Duration>,
        enquire_link_response_timeout: Duration,
        auto_enquire_link_response: bool,
    ) -> (Self, Sender<Request>, UnboundedReceiverStream<E::Event>) {
        // TODO: make this bound
        let (events_tx, events_rx) = unbounded_channel::<E::Event>();
        let events = E::new(events_tx);

        let (requests_tx, requests_rx) = channel::<Request>(100);

        let enquire_link_timer = enquire_link_interval
            .map(|duration| Timer::<D>::active(duration))
            .unwrap_or(Timer::<D>::inactive());

        let enquire_link_response_timer = Timer::<D>::inactive();

        (
            Self {
                sequence_number,
                framed,
                requests: ReceiverStream::new(requests_rx),
                events,
                responses: BTreeMap::new(),
                enquire_link_interval,
                enquire_link_response_timeout,
                enquire_link_timer,
                enquire_link_response_timer,
                auto_enquire_link_response,
            },
            requests_tx,
            UnboundedReceiverStream::new(events_rx),
        )
    }
}

impl<F, E: EventChannel, D: Delay> Connection2<F, E, D>
where
    F: Stream<Item = Result<Command, DecodeError>>
        + for<'a> Sink<&'a Command, Error = EncodeError>
        + Unpin,
{
    #[allow(clippy::collapsible_if)]
    async fn run(mut self) {
        let mut enquire_link_timer = pin!(self.enquire_link_timer);
        let mut enquire_link_response_timer = pin!(self.enquire_link_response_timer);

        loop {
            match ready(
                enquire_link_timer.as_mut(),
                enquire_link_response_timer.as_mut(),
                &mut self.framed,
                &mut self.requests,
            )
            .await
            {
                Ready::EnquireLinkTimer => {
                    let sequence_number = self.sequence_number.next();

                    let command = Command::builder()
                        .status(CommandStatus::EsmeRok)
                        .sequence_number(sequence_number)
                        .pdu(Pdu::EnquireLink);

                    if let Err(err) = self.framed.send(&command).await {
                        break;
                    }

                    self.responses.insert(
                        sequence_number,
                        Response::Connection(ConnectionCommandId::EnquireLink),
                    );

                    enquire_link_response_timer
                        .as_mut()
                        .activate(self.enquire_link_response_timeout);
                }
                Ready::EnquireLinkResponseTimer => {
                    // the response to our enquire link has not been received in time, so we can close the connection
                    break;
                }
                Ready::Framed(command) => {
                    // reset the enquire link timer since the connection is active
                    if let Some(delay) = self.enquire_link_interval {
                        enquire_link_timer.as_mut().activate(delay);
                    }

                    let command = match command {
                        Some(Ok(command)) => command,
                        Some(Err(err)) => {
                            break;
                        }
                        None => {
                            break;
                        }
                    };

                    if command.id().is_response() {
                        if let Some(response) = self.responses.remove(&command.sequence_number) {
                            match response {
                                Response::Client(sender) => {
                                    let _ = sender.send(command);
                                    // send through events if fails
                                }
                                Response::Connection(ConnectionCommandId::EnquireLink) => {
                                    enquire_link_response_timer.as_mut().deactivate();
                                }
                            }
                        }

                        // send through events if has no matching response
                    }

                    // send through events if has no matching response
                }
                Ready::Request(request) => {
                    let request = match request {
                        Some(request) => request,
                        None => {
                            break;
                        }
                    };

                    if let Err(err) = self.framed.send(&request.command).await {
                        break;
                    }

                    if let Some(response) = request.response {
                        self.responses
                            .insert(request.command.sequence_number, Response::Client(response));
                    }

                    let _ = request.ack.send(Ok(()));
                }
            };
        }
    }
}

enum Ready {
    EnquireLinkTimer,
    EnquireLinkResponseTimer,
    Framed(Option<Result<Command, DecodeError>>),
    Request(Option<Request>),
}

struct Request {
    command: Command,
    ack: oneshot::Sender<Result<(), Error>>,
    response: Option<oneshot::Sender<Command>>,
}

enum Response {
    /// A response that should be sent to the client
    Client(oneshot::Sender<Command>),
    /// A response for a command that was sent by the [`Connection`].
    Connection(ConnectionCommandId),
}

enum ConnectionCommandId {
    EnquireLink,
}

async fn ready<F, R, D>(
    mut enquire_link_timer: Pin<&mut Timer<D>>,
    mut enquire_link_response_timer: Pin<&mut Timer<D>>,
    framed: &mut F,
    requests: &mut R,
) -> Ready
where
    F: Stream<Item = Result<Command, DecodeError>>
        + for<'a> Sink<&'a Command, Error = EncodeError>
        + Unpin,
    R: Stream<Item = Request> + Unpin,
    D: Delay,
{
    match select(
        select(&mut enquire_link_timer, &mut enquire_link_response_timer),
        select(framed.next(), requests.next()),
    )
    .await
    {
        Either::Left((left, _)) => match left {
            Either::Left((_, _)) => Ready::EnquireLinkTimer,
            Either::Right((_, _)) => Ready::EnquireLinkResponseTimer,
        },
        Either::Right((right, _)) => match right {
            Either::Left((command, _)) => Ready::Framed(command),
            Either::Right((request, _)) => Ready::Request(request),
        },
    }
}
