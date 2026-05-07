/// Wasm runtime.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Wasm;

#[cfg(feature = "wasm")]
const _: () = {
    use std::{
        pin::Pin,
        task::{Context, Poll},
        time::Duration,
    };

    use super::{Delay, Timeout};

    impl Delay for Wasm {
        type Future = gloo_timers::future::TimeoutFuture;

        fn delay(duration: Duration) -> Self::Future {
            gloo_timers::future::sleep(duration)
        }
    }

    impl Timeout for Wasm {
        type Future<F: Future> = WasmTimeoutFuture<F>;

        fn timeout<F: Future>(duration: Duration, future: F) -> Self::Future<F> {
            WasmTimeoutFuture {
                future,
                delay: gloo_timers::future::sleep(duration),
            }
        }
    }

    pin_project_lite::pin_project! {
        /// The future returned by [`Wasm::timeout`].
        pub struct WasmTimeoutFuture<F> {
            #[pin]
            future: F,
            #[pin]
            delay: gloo_timers::future::TimeoutFuture,
        }
    }

    impl<F: Future> Future for WasmTimeoutFuture<F> {
        type Output = Option<F::Output>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.project();

            if let Poll::Ready(output) = this.future.poll(cx) {
                return Poll::Ready(Some(output));
            }

            match this.delay.poll(cx) {
                Poll::Ready(()) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            }
        }
    }

    impl Wasm {
        pub(crate) fn spawn<F>(future: F)
        where
            F: Future<Output = ()> + 'static,
        {
            wasm_bindgen_futures::spawn_local(future)
        }
    }
};
