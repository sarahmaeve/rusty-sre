use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Default)]
pub struct YieldOnce {
    yielded: bool,
}

impl YieldOnce {
    pub const fn new() -> Self {
        Self { yielded: false }
    }
}

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}
