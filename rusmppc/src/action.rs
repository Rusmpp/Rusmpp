use crate::{CloseRequest, PendingResponses, Request};

/// An [`Action`] is sent from the [`Client`](crate::Client) to the [`Connection`] to perform an action.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Action {
    /// A request issued by the [`Client`](crate::Client) to the [`Connection`].
    Request(Request),
    /// Sent from the [`Client`](crate::Client) to the [`Connection`] to check if the connection is closed or not.
    ///
    /// The [`Client`](crate::Client) would fail to send this action through the channel if the connection is closed.
    Ping,
    /// Retrieves pending responses from the connection.
    PendingResponses(PendingResponses),
}

impl Action {
    pub const fn request(request: Request) -> Self {
        Self::Request(request)
    }
}

/// An [`UrgentAction`] is sent from the [`Client`](crate::Client) to the [`Connection`] through a separate channel to perform an urgent action.
#[derive(Debug)]
pub enum UrgentAction {
    /// Removes a pending response from the [`Connection`]'s pending responses map.
    ///
    /// See [`RequestFutureGuard`](crate::futures::RequestFutureGuard).
    RemoveResponse(u32),
    /// After receiving the [`CloseRequest`] the [`Connection`] will close its channels and shutdown the connection and terminate.
    Close(CloseRequest),
}
