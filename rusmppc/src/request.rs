use rusmpp::Command;
use tokio::sync::oneshot;

use crate::error::Error;

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
    /// These requests are `not` stored in the connection's pending requests map and are not acknowledged.
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

    pub fn send_ack(self, ack: Result<(), Error>) -> Result<(), Result<(), Error>> {
        match self {
            Request::Registered(request) => request.ack.send(ack),
            Request::Unregistered(request) => request.ack.send(ack),
            Request::Obligated(_) => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RegisteredRequest {
    pub command: Command,
    /// ack result means that the command was sent, or could not be sent.
    pub ack: oneshot::Sender<Result<(), Error>>,
    /// response is a command sent from the server with a sequence number matching this command's sequence number.
    ///
    /// The background connection can only pass commands from the server with a matching sequence number without any validation.
    /// It's the client's responsibility to handle error commands.
    pub response: oneshot::Sender<Command>,
}

impl RegisteredRequest {
    pub fn new(
        command: Command,
    ) -> (
        Self,
        oneshot::Receiver<Result<(), Error>>,
        oneshot::Receiver<Command>,
    ) {
        let (ack, ack_rx) = oneshot::channel();
        let (response, response_rx) = oneshot::channel();

        (
            Self {
                command,
                ack,
                response,
            },
            ack_rx,
            response_rx,
        )
    }
}

#[derive(Debug)]
pub struct UnregisteredRequest {
    pub command: Command,
    /// ack result means that the command was sent, or could not be sent.
    pub ack: oneshot::Sender<Result<(), Error>>,
}

impl UnregisteredRequest {
    pub fn new(command: Command) -> (Self, oneshot::Receiver<Result<(), Error>>) {
        let (ack, rx) = oneshot::channel();

        (Self { command, ack }, rx)
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
