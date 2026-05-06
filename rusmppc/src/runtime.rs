use std::time::Duration;

pub mod tokio;

/// Abstraction over delay implementations for timers.
pub trait Delay {
    type Future: Future<Output = ()>;

    fn delay(duration: Duration) -> Self::Future;
}

/// Abstraction over timeout implementations for timers.
pub trait Timeout {
    type Future<F: Future>: Future<Output = Option<F::Output>>;

    fn timeout<F: Future>(duration: Duration, future: F) -> Self::Future<F>;
}
