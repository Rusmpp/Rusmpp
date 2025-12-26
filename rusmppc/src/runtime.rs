use std::time::Duration;

use crate::delay::Delay;

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn<F>(future: F) -> tokio::task::JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    tokio::spawn(future)
}

#[cfg(target_arch = "wasm32")]
pub fn spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future)
}

pub async fn timeout<F>(duration: Duration, future: F) -> Result<F::Output, ()>
where
    F: IntoFuture,
{
    #[cfg(not(target_arch = "wasm32"))]
    return tokio::time::timeout(duration, future).await.map_err(|_| ());

    #[cfg(target_arch = "wasm32")]
    {
        let delay = gloo_timers::future::sleep(duration);

        return TimeOut::new(delay, future.into_future()).await;
    }
}

/// Delay implementation using the runtime's timer.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RuntimeDelay;

impl RuntimeDelay {
    /// Creates a new [`RuntimeDelay`].
    pub const fn new() -> Self {
        Self
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Delay for RuntimeDelay {
    type Future = tokio::time::Sleep;

    fn delay(&self, duration: Duration) -> Self::Future {
        tokio::time::sleep(duration)
    }
}

#[cfg(target_arch = "wasm32")]
impl Delay for RuntimeDelay {
    type Future = gloo_timers::future::TimeoutFuture;

    fn delay(&self, duration: Duration) -> Self::Future {
        gloo_timers::future::sleep(duration)
    }
}

#[cfg(target_arch = "wasm32")]
pin_project_lite::pin_project! {
    struct TimeOut<D, F>{
        #[pin]
        delay: D,
        #[pin]
        future: F,
    }
}

#[cfg(target_arch = "wasm32")]
impl<D, F> TimeOut<D, F> {
    const fn new(delay: D, future: F) -> Self {
        Self { delay, future }
    }
}

#[cfg(target_arch = "wasm32")]
impl<D, F> Future for TimeOut<D, F>
where
    D: Future,
    F: Future,
{
    type Output = Result<F::Output, ()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut this = self.project();

        if this.delay.as_mut().poll(cx).is_ready() {
            return std::task::Poll::Ready(Err(()));
        }

        match this.future.as_mut().poll(cx) {
            std::task::Poll::Ready(val) => std::task::Poll::Ready(Ok(val)),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
