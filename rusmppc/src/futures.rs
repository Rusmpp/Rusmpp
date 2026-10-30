use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::sync::mpsc::UnboundedSender;

use crate::UrgentAction;

pin_project_lite::pin_project! {
    /// The [`ResponseFutureGuard`] is used to wrap a pending response future and remove its corresponding sequence number
    /// from the pending responses if the future got dropped.
    ///
    /// Why is removing the pending response so important even though the connection will pipe responses to the event stream anyway
    /// if sending the response to the client fails due to the receiving half being dropped/closed?
    ///
    /// * [`Client::pending_responses`](crate::Client::pending_responses) must be correct.
    /// * Prevent memory leaks. If the client sends a request and then the waiting future is dropped (using [`tokio::select!`]) and the server never responds to the sent request.
    ///     The pending response will stay in the connection's pending responses map and never gets removed.
    /// (response is never removed manually or because the server did not respond: memory leak).
    pub struct ResponseFutureGuard<'a, F> {
        done: bool,
        sequence_number: u32,
        actions: &'a UnboundedSender<UrgentAction>,
        #[pin]
        fut: F,
    }

    impl<F> PinnedDrop for ResponseFutureGuard<'_, F> {
        fn drop(this: Pin<&mut Self>) {
            let this = this.project();

            if !*this.done {
                let _ = this.actions
                    .send(UrgentAction::RemoveResponse(*this.sequence_number));
            }
        }
    }
}

impl<'a, F> ResponseFutureGuard<'a, F> {
    pub fn new(actions: &'a UnboundedSender<UrgentAction>, sequence_number: u32, fut: F) -> Self {
        Self {
            done: false,
            sequence_number,
            actions,
            fut,
        }
    }
}

impl<'a, F: Future> Future for ResponseFutureGuard<'a, F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.fut.poll(cx) {
            Poll::Ready(result) => {
                // Mark as done to prevent removing the sequence number on drop
                *this.done = true;

                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
