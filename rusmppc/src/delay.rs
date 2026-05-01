use std::time::Duration;

/// Abstraction over delay implementations for timers.
pub trait Delay: Default {
    type Future: Future<Output = ()>;

    fn delay(&self, duration: Duration) -> Self::Future;
}

/// Delay implementation using Tokio's timer.
#[derive(Debug, Default, Clone, Copy)]
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
