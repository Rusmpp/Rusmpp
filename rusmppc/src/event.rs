use rusmpp::Command;

use crate::error::Error;

/// `SMPP` event.
///
/// Events are sent from the open connection through the event stream.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Event {
    /// A command was received from the server.
    Incoming(Command),
    /// An error occurred.
    Error(Error),
}

impl Event {
    pub(crate) const fn incoming(command: Command) -> Self {
        Event::Incoming(command)
    }

    pub(crate) const fn error(error: Error) -> Self {
        Event::Error(error)
    }
}
