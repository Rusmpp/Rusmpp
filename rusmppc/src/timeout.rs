use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use pin_project::pin_project;

/// Abstraction over timeout implementations for timers.
pub trait Timeout {
    type Future<F: Future>: Future<Output = Option<F::Output>>;

    fn timeout<F: Future>(&'_ self, duration: Duration, future: F) -> Self::Future<F>;
}

/// Timeout implementation using Tokio's timer.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct TokioTimeout;

impl TokioTimeout {
    /// Creates a new [`TokioTimeout`].
    pub const fn new() -> Self {
        Self {}
    }
}

impl Timeout for TokioTimeout {
    type Future<F: Future> = TokioTimeoutFuture<F>;

    fn timeout<F: Future>(&self, duration: Duration, future: F) -> Self::Future<F> {
        TokioTimeoutFuture {
            future: tokio::time::timeout(duration, future),
        }
    }
}

pin_project_lite::pin_project! {
    /// The future returned by [`TokioTimeout::timeout`].
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

#[derive(Clone, Copy)]
pub enum TimeoutImpl {
    Tokio(TokioTimeout),
    #[cfg(test)]
    Mock(crate::mock::timeout::MockTimeout),
}

impl TimeoutImpl {
    pub const fn tokio() -> Self {
        TimeoutImpl::Tokio(TokioTimeout::new())
    }

    #[cfg(test)]
    pub const fn mock() -> Self {
        TimeoutImpl::Mock(crate::mock::timeout::MockTimeout::new())
    }
}

impl Timeout for TimeoutImpl {
    type Future<F: Future> = TimeoutFuture<F>;

    fn timeout<F: Future>(&self, duration: Duration, future: F) -> Self::Future<F> {
        match self {
            TimeoutImpl::Tokio(tokio_timeout) => {
                TimeoutFuture::Tokio(tokio_timeout.timeout(duration, future))
            }
            #[cfg(test)]
            TimeoutImpl::Mock(mock_timeout) => {
                TimeoutFuture::Mock(mock_timeout.timeout(duration, future))
            }
        }
    }
}

#[pin_project(project = TimeoutFutureProj)]
pub enum TimeoutFuture<F> {
    Tokio(#[pin] TokioTimeoutFuture<F>),
    #[cfg(test)]
    Mock(#[pin] crate::mock::timeout::MockTimeoutFuture<F>),
}

impl<F: Future> Future for TimeoutFuture<F> {
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            TimeoutFutureProj::Tokio(future) => future.poll(cx),
            #[cfg(test)]
            TimeoutFutureProj::Mock(future) => future.poll(cx),
        }
    }
}
