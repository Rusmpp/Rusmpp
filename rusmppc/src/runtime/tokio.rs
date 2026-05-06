use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use super::{Delay, Timeout};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Tokio;

impl Delay for Tokio {
    type Future = tokio::time::Sleep;

    fn delay(duration: Duration) -> Self::Future {
        tokio::time::sleep(duration)
    }
}

impl Timeout for Tokio {
    type Future<F: Future> = TokioTimeoutFuture<F>;

    fn timeout<F: Future>(duration: Duration, future: F) -> Self::Future<F> {
        TokioTimeoutFuture {
            future: tokio::time::timeout(duration, future),
        }
    }
}

pin_project_lite::pin_project! {
    /// The future returned by [`Tokio::timeout`].
    ///
    /// Maps the output of `tokio::time::timeout` to `Option<F::Output>`, where `None` indicates that the timeout elapsed before the future completed, and `Some(output)` indicates that the future completed before the timeout with the given output.
    pub struct TokioTimeoutFuture<F> {
        #[pin]
        future: tokio::time::Timeout<F>,
    }
}

impl<F: Future> Future for TokioTimeoutFuture<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.future.poll(cx) {
            Poll::Ready(Ok(output)) => Poll::Ready(Some(output)),
            Poll::Ready(Err(_)) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
