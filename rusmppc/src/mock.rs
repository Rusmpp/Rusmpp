//! Mocks for Io, Stream/Sink, Delay and Timeout used in tests.

use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub mod io {
    use std::io;

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    use super::*;

    /// AsyncRead and AsyncWrite mock for IO streams.
    #[mockall::automock]
    pub trait Io {
        fn poll_read_pin<'a, 'b>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
            buf: &mut ReadBuf<'b>,
        ) -> Poll<io::Result<()>>;

        fn poll_write_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
            buf: &[u8],
        ) -> Poll<Result<usize, io::Error>>;

        fn poll_flush_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
        ) -> Poll<Result<(), io::Error>>;

        fn poll_shutdown_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
        ) -> Poll<Result<(), io::Error>>;
    }

    impl AsyncRead for MockIo {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            self.poll_read_pin(cx, buf)
        }
    }

    impl AsyncWrite for MockIo {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, io::Error>> {
            self.poll_write_pin(cx, buf)
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
            self.poll_flush_pin(cx)
        }

        fn poll_shutdown(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), io::Error>> {
            self.poll_shutdown_pin(cx)
        }
    }
}

pub mod framed {
    use futures::{Sink, Stream};
    use rusmpp::{Command, tokio_codec::DecodeError, tokio_codec::EncodeError};

    use super::*;

    /// Stream and Sink mock for [`Command`] framers.
    #[mockall::automock]
    pub trait Framed {
        fn poll_next_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
        ) -> Poll<Option<Result<Command, DecodeError>>>;

        fn poll_ready_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
        ) -> Poll<Result<(), EncodeError>>;

        fn start_send_pin(self: Pin<&mut Self>, item: &Command) -> Result<(), EncodeError>;

        fn poll_flush_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
        ) -> Poll<Result<(), EncodeError>>;

        fn poll_close_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
        ) -> Poll<Result<(), EncodeError>>;
    }

    impl Stream for MockFramed {
        type Item = Result<Command, DecodeError>;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            self.poll_next_pin(cx)
        }
    }

    impl Sink<&Command> for MockFramed {
        type Error = EncodeError;

        fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.poll_ready_pin(cx)
        }

        fn start_send(self: Pin<&mut Self>, item: &Command) -> Result<(), Self::Error> {
            self.start_send_pin(item)
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.poll_flush_pin(cx)
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.poll_close_pin(cx)
        }
    }

    impl MockFramed {
        pub fn poll_next_always_pending(mut self) -> MockFramed {
            self.expect_poll_next_pin().returning(|cx| {
                cx.waker().wake_by_ref();
                Poll::Pending
            });
            self
        }

        pub fn poll_ready_always_ready_ok(mut self) -> MockFramed {
            self.expect_poll_ready_pin()
                .returning(|_cx| Poll::Ready(Ok(())));
            self
        }

        pub fn poll_start_send_always_ok(mut self) -> MockFramed {
            self.expect_start_send_pin().returning(|_item| Ok(()));
            self
        }

        pub fn poll_flush_always_ready_ok(mut self) -> MockFramed {
            self.expect_poll_flush_pin()
                .returning(|_cx| Poll::Ready(Ok(())));
            self
        }

        pub fn poll_close_always_ready_ok(mut self) -> MockFramed {
            self.expect_poll_close_pin()
                .returning(|_cx| Poll::Ready(Ok(())));
            self
        }

        pub fn sink_always_ready_ok(self) -> MockFramed {
            self.poll_ready_always_ready_ok()
                .poll_start_send_always_ok()
                .poll_flush_always_ready_ok()
                .poll_close_always_ready_ok()
        }
    }

    #[test]
    fn test_sink_always_ready_ok() {
        let mut mock_framed = MockFramed::new().sink_always_ready_ok();

        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        let mut pinned = Pin::new(&mut mock_framed);

        for _ in 0..5 {
            let result = pinned.as_mut().poll_ready(&mut cx);
            assert!(matches!(result, Poll::Ready(Ok(()))));

            let result = pinned.as_mut().start_send(&Command::default());
            assert!(matches!(result, Ok(())));

            let result = pinned.as_mut().poll_flush(&mut cx);
            assert!(matches!(result, Poll::Ready(Ok(()))));

            let result = pinned.as_mut().poll_close(&mut cx);
            assert!(matches!(result, Poll::Ready(Ok(()))));
        }
    }
}

pub mod delay {
    use super::*;

    /// Delay mock for timers.
    ///
    /// This mock translates each second in the requested duration to one poll before completion.
    #[derive(Debug, Clone, Copy, Default)]
    #[non_exhaustive]
    pub struct MockDelay;

    impl crate::delay::Delay for MockDelay {
        type Future = MockDelayFuture;

        fn delay(&self, duration: Duration) -> Self::Future {
            MockDelayFuture::new(duration.as_secs())
        }
    }

    impl MockDelay {
        /// Creates a new [`MockDelay`].
        pub const fn new() -> Self {
            Self {}
        }
    }

    /// Future returned by the [`MockDelay`].
    ///
    /// Each poll corresponds to one second in the requested duration.
    pub struct MockDelayFuture {
        complete: bool,
        /// Number of polls before completion.
        after: u64,
    }

    impl MockDelayFuture {
        pub const fn new(after: u64) -> Self {
            Self {
                complete: false,
                after,
            }
        }
    }

    impl Future for MockDelayFuture {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.complete {
                panic!("polled after completion");
            }

            if self.after == 0 {
                self.complete = true;

                Poll::Ready(())
            } else {
                self.after -= 1;

                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    #[test]
    fn test_delay_always_after() {
        use crate::delay::Delay;

        let mock_delay = MockDelay::new();

        let mut delay_future = mock_delay.delay(Duration::from_secs(3));

        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        let mut pinned = Pin::new(&mut delay_future);

        for i in 0..5 {
            let result = pinned.as_mut().poll(&mut cx);

            if i < 3 {
                assert!(matches!(result, Poll::Pending));
            } else {
                assert!(matches!(result, Poll::Ready(())));
                break;
            }
        }
    }
}

pub mod timeout {
    use super::*;

    /// Timeout mock for timers.
    ///
    /// This mock translates each second in the requested duration to one poll before completion.
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct MockTimeout;

    impl MockTimeout {
        /// Creates a new [`MockTimeout`].
        pub const fn new() -> Self {
            Self {}
        }
    }

    impl crate::timeout::Timeout for MockTimeout {
        type Future<F: Future> = MockTimeoutFuture<F>;

        fn timeout<F: Future>(&self, duration: Duration, future: F) -> Self::Future<F> {
            MockTimeoutFuture {
                future,
                delay: delay::MockDelayFuture::new(duration.as_secs()),
            }
        }
    }

    pin_project_lite::pin_project! {
        /// Future returned by the [`MockTimeout`].
        ///
        /// Each poll corresponds to one second in the requested duration.
        pub struct MockTimeoutFuture<F> {
            #[pin]
            future: F,
            #[pin]
            delay: delay::MockDelayFuture,
        }
    }

    impl<F: Future> Future for MockTimeoutFuture<F> {
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

    /// Helper future that polls a given number of times before completion.
    fn poll_future(times: usize) -> impl Future<Output = ()> {
        let mut remaining = times;

        futures::future::poll_fn(move |cx| {
            if remaining == 0 {
                Poll::Ready(())
            } else {
                remaining -= 1;

                cx.waker().wake_by_ref();

                Poll::Pending
            }
        })
    }

    #[test]
    fn would_timeout() {
        use crate::timeout::Timeout;

        let three_polls_future = poll_future(3);

        let mock_timeout = MockTimeout::new();
        let mut timeout_future = mock_timeout.timeout(Duration::from_secs(2), three_polls_future);

        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        let mut pinned = Pin::new(&mut timeout_future);

        loop {
            let result = pinned.as_mut().poll(&mut cx);

            if let Poll::Ready(output) = result {
                assert!(output.is_none());

                break;
            }
        }
    }

    #[test]
    fn would_not_timeout() {
        use crate::timeout::Timeout;

        let three_polls_future = poll_future(3);

        let mock_timeout = MockTimeout::new();
        let mut timeout_future = mock_timeout.timeout(Duration::from_secs(5), three_polls_future);

        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        let mut pinned = Pin::new(&mut timeout_future);

        loop {
            let result = pinned.as_mut().poll(&mut cx);

            if let Poll::Ready(output) = result {
                assert!(output.is_some());

                break;
            }
        }
    }
}
