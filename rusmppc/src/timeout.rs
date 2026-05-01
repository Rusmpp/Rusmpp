use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

/// Abstraction over timeout implementations for timers.
pub trait Timeout<F: Future>: Default {
    type Future: Future<Output = Option<F::Output>>;

    fn timeout(&self, duration: Duration, future: F) -> Self::Future;
}

/// Timeout implementation using Tokio's timer.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct TokioTimeout;

impl TokioTimeout {
    /// Creates a new [`TokioTimeout`].
    pub const fn new() -> Self {
        Self
    }
}

impl<F: Future> Timeout<F> for TokioTimeout {
    type Future = TimeoutFuture<F>;

    fn timeout(&self, duration: Duration, future: F) -> Self::Future {
        TimeoutFuture {
            future: tokio::time::timeout(duration, future),
        }
    }
}

pin_project_lite::pin_project! {
    /// The future returned by [`TokioTimeout::timeout`].
    ///
    /// Maps the output of `tokio::time::timeout` to `Option<F::Output>`, where `None` indicates that the timeout elapsed before the future completed, and `Some(output)` indicates that the future completed before the timeout with the given output.
    pub struct TimeoutFuture<F> {
        #[pin]
        future: tokio::time::Timeout<F>,
    }
}

impl<F: Future> Future for TimeoutFuture<F> {
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
