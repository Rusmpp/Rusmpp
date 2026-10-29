use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Sink, Stream};

pin_project_lite::pin_project! {
    pub struct FairStream<S> {
        #[pin]
        stream: S,
        polls: usize,
        current_poll: usize,
    }
}

impl<S> FairStream<S> {
    pub fn new(stream: S, polls: usize) -> Self {
        assert!(polls > 0, "FairStream: polls must be > 0");

        Self {
            stream,
            polls,
            current_poll: 0,
        }
    }

    pub fn inner_mut(self: Pin<&mut Self>) -> Pin<&mut S> {
        self.project().stream
    }
}

impl<S: Stream> Stream for FairStream<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if *this.current_poll >= *this.polls {
            *this.current_poll = 0;

            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        match this.stream.poll_next(cx) {
            Poll::Ready(None) => Poll::Ready(None),

            Poll::Ready(Some(item)) => {
                *this.current_poll += 1;
                Poll::Ready(Some(item))
            }

            Poll::Pending => {
                *this.current_poll = 0;

                Poll::Pending
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

impl<S, Item> Sink<Item> for FairStream<S>
where
    S: Sink<Item>,
{
    type Error = S::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().stream.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        self.project().stream.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().stream.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().stream.poll_close(cx)
    }
}
