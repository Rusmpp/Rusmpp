use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Sender<T> {
    Bounded(mpsc::Sender<T>),
    Unbounded(mpsc::UnboundedSender<T>),
}

impl<T> Sender<T> {
    pub async fn send(&self, value: T) -> Result<(), mpsc::error::SendError<T>> {
        match self {
            Sender::Bounded(sender) => sender.send(value).await,
            Sender::Unbounded(sender) => sender.send(value),
        }
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        match self {
            Sender::Bounded(s) => Sender::Bounded(s.clone()),
            Sender::Unbounded(s) => Sender::Unbounded(s.clone()),
        }
    }
}

pin_project_lite::pin_project! {
    #[project = ReceiverStreamProj]
    pub enum ReceiverStream<T> {
        Bounded {
            #[pin]
            inner: tokio_stream::wrappers::ReceiverStream<T>,
        },
        Unbounded {
            #[pin]
            inner: tokio_stream::wrappers::UnboundedReceiverStream<T>,
        },
    }
}

impl<T> ReceiverStream<T> {
    /// Closes the receiving half of the channel without dropping it.
    ///
    /// No more values can be sent after this, but values already in the
    /// buffer can still be received. See [`mpsc::Receiver::close`].
    pub fn close(self: Pin<&mut Self>) {
        match self.project() {
            ReceiverStreamProj::Bounded { inner } => inner.get_mut().close(),
            ReceiverStreamProj::Unbounded { inner } => inner.get_mut().close(),
        }
    }
}

impl<T> Stream for ReceiverStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.project() {
            ReceiverStreamProj::Bounded { inner } => inner.poll_next(cx),
            ReceiverStreamProj::Unbounded { inner } => inner.poll_next(cx),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            ReceiverStream::Bounded { inner } => inner.size_hint(),
            ReceiverStream::Unbounded { inner } => inner.size_hint(),
        }
    }
}

/// Creates a new channel with the given capacity. If `capacity` is `None`, an unbounded channel is created.
///
/// # Panics
///
/// Panics if `capacity` is `Some(0)`.
pub fn channel<T>(capacity: Option<usize>) -> (Sender<T>, ReceiverStream<T>) {
    assert!(
        capacity.is_none_or(|c| c > 0),
        "Channel capacity must be greater than 0"
    );

    match capacity {
        Some(capacity) => {
            let (tx, rx) = mpsc::channel(capacity);
            (
                Sender::Bounded(tx),
                ReceiverStream::Bounded {
                    inner: tokio_stream::wrappers::ReceiverStream::new(rx),
                },
            )
        }
        None => {
            let (tx, rx) = mpsc::unbounded_channel();
            (
                Sender::Unbounded(tx),
                ReceiverStream::Unbounded {
                    inner: tokio_stream::wrappers::UnboundedReceiverStream::new(rx),
                },
            )
        }
    }
}
