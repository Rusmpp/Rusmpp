use crate::{CloseRequest, PendingResponses, RegisteredRequest, Request, UnregisteredRequest};

/// Action
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Action {
    /// Sends a request to the server.
    Request(Request),
    /// Removes a pending response from the connection's pending responses map.
    ///
    /// See [`RequestFutureGuard`](crate::futures::RequestFutureGuard).
    Remove(u32),
    /// The connection will stop reading from the server, stop time keeping, close the requests channel, flush pending requests and terminate.
    Close(CloseRequest),
    /// Sent from the client to the connection to check if the connection is closed or not.
    ///
    /// The client would fail to send this action through the channel if the connection is closed.
    Ping,
    /// Retrieves pending responses from the connection.
    PendingResponses(PendingResponses),
}

impl Action {
    /// Sends a registered request to the server.
    pub const fn registered_request(request: RegisteredRequest) -> Self {
        Self::Request(Request::Registered(request))
    }

    /// Sends an unregistered request to the server.
    pub const fn unregistered_request(request: UnregisteredRequest) -> Self {
        Self::Request(Request::Unregistered(request))
    }
}
