use crate::{
    CloseRequest, PendingResponses, RegisteredRequest, Request, RequestId, UnregisteredRequest,
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Action {
    Request(Request),
    /// Drops a queued request that its caller abandoned before it was written.
    ///
    /// A cleanup hint, never the authority: the request's own state cell decides what a
    /// cancellation meant, and the write gate refuses an abandoned request regardless of
    /// whether this action arrives. The hint only lets the connection release the queue
    /// entry (and its memory) as soon as it notices, instead of dragging it until the
    /// request reaches the gate.
    ///
    /// See [`RequestFutureGuard`](crate::futures::RequestFutureGuard).
    Cancel(RequestId),
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
    pub const fn registered_request(request: RegisteredRequest) -> Self {
        Self::Request(Request::Registered(request))
    }

    pub const fn unregistered_request(request: UnregisteredRequest) -> Self {
        Self::Request(Request::Unregistered(request))
    }
}
