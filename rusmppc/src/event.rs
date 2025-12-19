use std::fmt::Debug;

use rusmpp::Command;
use tokio::sync::mpsc::error::SendError;

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

/// `SMPP` event with insights
///
/// Events are sent from the open connection through the event stream.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum InsightEvent {
    /// A command was received from the server.
    Incoming(Command),
    /// An error occurred.
    Error(Error),
    /// An insight event.
    Insight(Insight),
}

/// Connection insight event.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Insight {
    /// Sent EnquireLink command to the server.
    SentEnquireLink(u32),
    /// Received EnquireLinkResp from the server.
    ReceivedEnquireLinkResp(u32),
    /// Received EnquireLink command from the server.
    ReceivedEnquireLink(u32),
    /// Sent EnquireLinkResp to the server.
    SentEnquireLinkResp(u32),
}

pub trait EventChannel: Send + 'static {
    type Event;

    /// Creates a new [`EventChannel`] instance.
    fn new(sender: tokio::sync::mpsc::UnboundedSender<Self::Event>) -> Self
    where
        Self: Sized;

    /// Sends an [`Error`] event through the event channel.
    fn send_error(&self, error: Error) -> Result<(), SendError<Self::Event>>;

    /// Sends an incoming [`Command`] event through the event channel.
    fn send_incoming(&self, command: Command) -> Result<(), SendError<Self::Event>>;

    /// Sends an [`Insight`] event through the event channel.
    fn send_insight(&self, insight: Insight) -> Result<(), SendError<Self::Event>>;
}

/// The default [`EventChannel`] implementation that sends [`Event`]s through the event stream.
pub struct DefaultEventChannel {
    sender: tokio::sync::mpsc::UnboundedSender<Event>,
}

impl Debug for DefaultEventChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultEventChannel").finish()
    }
}

impl EventChannel for DefaultEventChannel {
    type Event = Event;

    fn new(sender: tokio::sync::mpsc::UnboundedSender<Self::Event>) -> Self {
        Self { sender }
    }

    fn send_error(&self, error: Error) -> Result<(), SendError<Self::Event>> {
        self.sender.send(Event::Error(error))
    }

    fn send_incoming(&self, command: Command) -> Result<(), SendError<Self::Event>> {
        self.sender.send(Event::Incoming(command))
    }

    fn send_insight(&self, _insight: Insight) -> Result<(), SendError<Self::Event>> {
        // Noop for default event channel

        Ok(())
    }
}

/// An [`EventChannel`] implementation that discards all events.
pub struct DiscardEventChannel {
    // even if we don't use it, we should keep it alive to avoid closing the event stream
    // closing the event stream is associated with the connection being closed/dropped
    _sender: tokio::sync::mpsc::UnboundedSender<()>,
}

impl Debug for DiscardEventChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiscardEventChannel").finish()
    }
}

impl EventChannel for DiscardEventChannel {
    type Event = ();

    fn new(sender: tokio::sync::mpsc::UnboundedSender<Self::Event>) -> Self {
        Self { _sender: sender }
    }

    fn send_error(&self, _error: Error) -> Result<(), SendError<Self::Event>> {
        Ok(())
    }

    fn send_incoming(&self, _command: Command) -> Result<(), SendError<Self::Event>> {
        Ok(())
    }

    fn send_insight(&self, _insight: Insight) -> Result<(), SendError<Self::Event>> {
        Ok(())
    }
}

/// An [`EventChannel`] implementation that sends [`InsightEvent`]s through the event stream.
pub struct InsightEventChannel {
    sender: tokio::sync::mpsc::UnboundedSender<InsightEvent>,
}

impl Debug for InsightEventChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InsightEventChannel").finish()
    }
}

impl EventChannel for InsightEventChannel {
    type Event = InsightEvent;

    fn new(sender: tokio::sync::mpsc::UnboundedSender<Self::Event>) -> Self {
        Self { sender }
    }

    fn send_error(&self, error: Error) -> Result<(), SendError<Self::Event>> {
        self.sender.send(InsightEvent::Error(error))
    }

    fn send_incoming(&self, command: Command) -> Result<(), SendError<Self::Event>> {
        self.sender.send(InsightEvent::Incoming(command))
    }

    fn send_insight(&self, insight: Insight) -> Result<(), SendError<Self::Event>> {
        self.sender.send(InsightEvent::Insight(insight))
    }
}
