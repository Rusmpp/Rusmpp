use crate::{CloseRequest, PendingResponses, Request};

/// An [`Action`] is sent from the [`Client`](crate::Client) to the [`Connection`](crate::connection::Connection) to perform an action.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Action {
    /// A request issued by the [`Client`](crate::Client) to the [`Connection`](crate::connection::Connection).
    Request(Request),
    /// Retrieves pending responses from the connection.
    PendingResponses(PendingResponses),
}

impl Action {
    pub const fn request(request: Request) -> Self {
        Self::Request(request)
    }
}

/// An [`UrgentAction`] is sent from the [`Client`](crate::Client) to the [`Connection`](crate::connection::Connection) through a separate channel to perform an urgent action.
#[derive(Debug)]
pub enum UrgentAction {
    /// Removes a pending response from the [`Connection`](crate::connection::Connection)'s pending responses map.
    ///
    /// See [`ResponseFutureGuard`](crate::futures::ResponseFutureGuard).
    RemoveResponse(u32),
    /// After receiving the [`CloseRequest`] the [`Connection`](crate::connection::Connection) will close its channels and shutdown the connection and terminate.
    Close(CloseRequest),
}
