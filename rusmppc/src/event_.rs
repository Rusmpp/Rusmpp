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
    fn new(sender: crate::sync::Sender<Self::Event>) -> Self
    where
        Self: Sized;

    /// Sends an [`Error`] event through the event channel.
    fn send_error(
        &self,
        error: Error,
    ) -> impl Future<Output = Result<(), SendError<Self::Event>>> + Send;

    /// Sends an incoming [`Command`] event through the event channel.
    fn send_incoming(
        &self,
        command: Command,
    ) -> impl Future<Output = Result<(), SendError<Self::Event>>> + Send;

    /// Sends an [`Insight`] event through the event channel.
    fn send_insight(
        &self,
        insight: Insight,
    ) -> impl Future<Output = Result<(), SendError<Self::Event>>> + Send;
}

/// The default `EventChannel` implementation that sends [`Event`]s through the event stream.
#[derive(Clone)]
pub struct DefaultEventChannel {
    sender: crate::sync::Sender<Event>,
}

impl Debug for DefaultEventChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultEventChannel").finish()
    }
}

impl EventChannel for DefaultEventChannel {
    type Event = Event;

    fn new(sender: crate::sync::Sender<Self::Event>) -> Self {
        Self { sender }
    }

    async fn send_error(&self, error: Error) -> Result<(), SendError<Self::Event>> {
        self.sender.send(Event::Error(error)).await
    }

    async fn send_incoming(&self, command: Command) -> Result<(), SendError<Self::Event>> {
        self.sender.send(Event::Incoming(command)).await
    }

    async fn send_insight(&self, _insight: Insight) -> Result<(), SendError<Self::Event>> {
        // Noop for default event channel

        Ok(())
    }
}

/// An `EventChannel` implementation that discards all events.
#[derive(Clone)]
pub struct DiscardEventChannel {
    // even if we don't use it, we should keep it alive to avoid closing the event stream
    // closing the event stream is associated with the connection being closed/dropped
    _sender: crate::sync::Sender<()>,
}

impl Debug for DiscardEventChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiscardEventChannel").finish()
    }
}

impl EventChannel for DiscardEventChannel {
    type Event = ();

    fn new(sender: crate::sync::Sender<Self::Event>) -> Self {
        Self { _sender: sender }
    }

    async fn send_error(&self, _error: Error) -> Result<(), SendError<Self::Event>> {
        Ok(())
    }

    async fn send_incoming(&self, _command: Command) -> Result<(), SendError<Self::Event>> {
        Ok(())
    }

    async fn send_insight(&self, _insight: Insight) -> Result<(), SendError<Self::Event>> {
        Ok(())
    }
}

/// An `EventChannel` implementation that sends [`InsightEvent`]s through the event stream.
#[derive(Clone)]
pub struct InsightEventChannel {
    sender: crate::sync::Sender<InsightEvent>,
}

impl Debug for InsightEventChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InsightEventChannel").finish()
    }
}

impl EventChannel for InsightEventChannel {
    type Event = InsightEvent;

    fn new(sender: crate::sync::Sender<Self::Event>) -> Self {
        Self { sender }
    }

    async fn send_error(&self, error: Error) -> Result<(), SendError<Self::Event>> {
        self.sender.send(InsightEvent::Error(error)).await
    }

    async fn send_incoming(&self, command: Command) -> Result<(), SendError<Self::Event>> {
        self.sender.send(InsightEvent::Incoming(command)).await
    }

    async fn send_insight(&self, insight: Insight) -> Result<(), SendError<Self::Event>> {
        self.sender.send(InsightEvent::Insight(insight)).await
    }
}
