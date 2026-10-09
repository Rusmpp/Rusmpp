use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::sync::mpsc::UnboundedSender;

use crate::{Action, RequestCell, RequestId};

pin_project_lite::pin_project! {
    /// The [`RequestFutureGuard`] wraps a pending request future and gives the request's
    /// state cell its cancellation transition if the future is dropped before it completes.
    ///
    /// The guard is what makes "definitely not sent" independent of the connection: the
    /// cell's `Queued -> Abandoned` transition happens synchronously here, in the caller's
    /// task, and the write gate refuses an abandoned request — so a cancelled send never
    /// reaches the peer, even when the connection is busy, starved or gone.
    ///
    /// If the request had already been handed to the sink, the cell records that instead:
    /// the request may be out, and its sequence number stays reserved (unresolved) until
    /// the response arrives. And if a response is already committed but the caller never
    /// consumed it, the drop takes it and routes it late — the reply is owed to the
    /// application even when the caller walked away. The [`Action::Cancel`] is only a
    /// cleanup hint that lets the connection drop the queued entry early; it is never the
    /// authority.
    pub struct RequestFutureGuard<'a, F> {
        done: bool,
        id: RequestId,
        cell: RequestCell,
        actions: &'a UnboundedSender<Action>,
        #[pin]
        fut: F,
    }

    impl<F> PinnedDrop for RequestFutureGuard<'_, F> {
        fn drop(this: Pin<&mut Self>) {
            let this = this.project();

            if !*this.done {
                // The cell is the arbiter, and it settles this atomically: a response
                // committed but not yet consumed is taken here and routed late — the
                // caller is gone, and the reply is still owed to the application. When
                // nothing is stored, the request is abandoned synchronously, so the write
                // gate refuses it if it is still queued.
                if let Some(command) = this.cell.abandon_or_take() {
                    this.cell.forward_late(command);
                }

                let _ = this.actions.send(Action::Cancel(*this.id));
            }
        }
    }
}

impl<'a, F> RequestFutureGuard<'a, F> {
    pub fn new(
        actions: &'a UnboundedSender<Action>,
        id: RequestId,
        cell: RequestCell,
        fut: F,
    ) -> Self {
        Self {
            done: false,
            id,
            cell,
            actions,
            fut,
        }
    }
}

impl<'a, F: Future> Future for RequestFutureGuard<'a, F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.fut.poll(cx) {
            Poll::Ready(result) => {
                // Mark as done to prevent abandoning the request on drop: the outcome has
                // already been decided.
                *this.done = true;

                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
