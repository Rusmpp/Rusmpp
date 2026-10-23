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
use tokio::sync::oneshot;

use crate::{Action, SequenceNumber, Timer, error::Error, runtime_::Delay};

enum Ready {
    EnquireLinkTimer,
    EnquireLinkResponseTimer,
    Framed(Option<Result<Command, DecodeError>>),
    Request(Option<Request>),
}

#[allow(clippy::collapsible_if)]
#[allow(clippy::single_match)]
pub async fn run<F, R, D>(
    sequence_number: SequenceNumber,
    mut framed: F,
    mut requests: R,
    enquire_link_interval: Option<Duration>,
    enquire_link_response_timeout: Duration,
) where
    F: Stream<Item = Result<Command, DecodeError>>
        + for<'a> Sink<&'a Command, Error = EncodeError>
        + Unpin,
    R: Stream<Item = Request> + Unpin,
    D: Delay,
{
    let mut enquire_link_timer = pin!(
        enquire_link_interval
            .map(|duration| Timer::<D>::active(duration))
            .unwrap_or(Timer::<D>::inactive())
    );

    let mut enquire_link_response_timer = pin!(Timer::<D>::inactive());

    let mut responses: BTreeMap<u32, Response> = BTreeMap::new();

    loop {
        match ready(
            enquire_link_timer.as_mut(),
            enquire_link_response_timer.as_mut(),
            &mut framed,
            &mut requests,
        )
        .await
        {
            Ready::EnquireLinkTimer => {
                let sequence_number = sequence_number.next();

                let command = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(sequence_number)
                    .pdu(Pdu::EnquireLink);

                if let Err(err) = framed.send(&command).await {
                    break;
                }

                responses.insert(
                    sequence_number,
                    Response::Connection(ConnectionCommandId::EnquireLink),
                );

                enquire_link_response_timer
                    .as_mut()
                    .activate(enquire_link_response_timeout);
            }
            Ready::EnquireLinkResponseTimer => {
                // the response to our enquire link has not been received in time, so we can close the connection
                break;
            }
            Ready::Framed(command) => {
                // reset the enquire link timer since the connection is active
                if let Some(delay) = enquire_link_interval {
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
                    if let Some(response) = responses.remove(&command.sequence_number) {
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
                }

                // send through events if has no matching response (wtf)
            }
            Ready::Request(request) => {
                let request = match request {
                    Some(request) => request,
                    None => {
                        break;
                    }
                };

                if let Err(err) = framed.send(&request.command).await {
                    break;
                }

                if let Some(response) = request.response {
                    responses.insert(request.command.sequence_number, Response::Client(response));
                }

                let _ = request.ack.send(Ok(()));
            }
        };
    }
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
