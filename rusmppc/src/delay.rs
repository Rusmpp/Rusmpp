use std::time::Duration;

/// Abstraction over delay implementations for timers.
pub trait Delay: Default {
    type Future: Future<Output = ()>;

    fn delay(&self, duration: Duration) -> Self::Future;
}
