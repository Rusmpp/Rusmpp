use rusmpp::Command;
use tokio::sync::oneshot;

use crate::error::Error;

/// A request issued by the [`Client`](crate::Client) to the [`Connection`].
///
/// If the [`Client`](crate::Client) want to wait for a matching response from the server, [`Request::response`] should be set to `Some`.
///
/// In this case, the [`Connection`] will store the response in the pending responses map, and will send the response to the [`Client`](crate::Client) when a matching response is received.
#[derive(Debug)]
pub struct Request {
    /// The command to be sent to the server.
    pub command: Command,
    /// The request is being sent to the server.
    pub start_send: oneshot::Sender<()>,
    /// The request has been sent to the server.
    pub sent: oneshot::Sender<Result<(), Error>>,
    /// Response is a command sent from the server with a sequence number matching this command's sequence number.
    ///
    /// The [`Connection`] can only pass responses from the server with a matching sequence number without any validation.
    ///
    /// It's the [`Client`](crate::Client)'s responsibility to handle unexpected responses.
    pub response: Option<oneshot::Sender<Command>>,
}

impl Request {
    #[allow(clippy::type_complexity)]
    pub fn registered(
        command: Command,
    ) -> (
        Self,
        oneshot::Receiver<()>,
        oneshot::Receiver<Result<(), Error>>,
        oneshot::Receiver<Command>,
    ) {
        let (start_send, start_send_rx) = oneshot::channel();
        let (sent, sent_rx) = oneshot::channel();
        let (response, response_rx) = oneshot::channel();

        (
            Self {
                command,
                start_send,
                sent,
                response: Some(response),
            },
            start_send_rx,
            sent_rx,
            response_rx,
        )
    }

    pub fn unregistered(
        command: Command,
    ) -> (
        Self,
        oneshot::Receiver<()>,
        oneshot::Receiver<Result<(), Error>>,
    ) {
        let (start_send, start_send_rx) = oneshot::channel();
        let (sent, sent_rx) = oneshot::channel();

        (
            Self {
                command,
                start_send,
                sent,
                response: None,
            },
            start_send_rx,
            sent_rx,
        )
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
