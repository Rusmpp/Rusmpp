use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use pin_project_lite::pin_project;

use crate::delay::Delay;

pin_project! {
    #[derive(Debug)]
    pub struct Timer<D: Delay> {
        delay: D,
        #[pin]
        state: Option<D::Future>,
    }
}

impl<D: Delay> Timer<D> {
    pub const fn inactive(delay: D) -> Self {
        Self { delay, state: None }
    }

    pub fn active(delay: D, duration: Duration) -> Self {
        Self::inactive(delay).activated(duration)
    }

    pub fn activated(mut self, duration: Duration) -> Timer<D> {
        self.state = Some(self.delay.delay(duration));
        self
    }

    pub fn deactivate(self: Pin<&mut Self>) {
        self.project().state.set(None);
    }

    pub fn activate(self: Pin<&mut Self>, duration: Duration) {
        let delay = self.delay.delay(duration);

        self.project().state.set(Some(delay));
    }
}

impl<D: Delay> Future for Timer<D> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .state
            .as_pin_mut()
            .map(|delay| delay.poll(cx))
            .unwrap_or(Poll::Pending)
    }
}
