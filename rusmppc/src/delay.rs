use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use pin_project::pin_project;

/// Abstraction over delay implementations for timers.
pub trait Delay {
    type Future: Future<Output = ()>;

    fn delay(&self, duration: Duration) -> Self::Future;
}

/// Delay implementation using Tokio's timer.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct TokioDelay;

impl TokioDelay {
    /// Creates a new [`TokioDelay`].
    pub const fn new() -> Self {
        Self
    }
}

impl Delay for TokioDelay {
    type Future = tokio::time::Sleep;

    fn delay(&self, duration: Duration) -> Self::Future {
        tokio::time::sleep(duration)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DelayImpl {
    Tokio(TokioDelay),
    #[cfg(test)]
    Mock(crate::mock::delay::MockDelay),
}

impl DelayImpl {
    pub const fn tokio() -> Self {
        DelayImpl::Tokio(TokioDelay::new())
    }

    #[cfg(test)]
    pub const fn mock() -> Self {
        DelayImpl::Mock(crate::mock::delay::MockDelay::new())
    }
}

impl Delay for DelayImpl {
    type Future = DelayFuture;

    fn delay(&self, duration: Duration) -> Self::Future {
        match self {
            DelayImpl::Tokio(delay) => DelayFuture::Tokio(delay.delay(duration)),
            #[cfg(test)]
            DelayImpl::Mock(delay) => DelayFuture::Mock(delay.delay(duration)),
        }
    }
}

#[pin_project(project = DelayFutureProj)]
pub enum DelayFuture {
    Tokio(#[pin] tokio::time::Sleep),
    #[cfg(test)]
    Mock(#[pin] crate::mock::delay::MockDelayFuture),
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            DelayFutureProj::Tokio(future) => future.poll(cx),
            #[cfg(test)]
            DelayFutureProj::Mock(future) => future.poll(cx),
        }
    }
}
