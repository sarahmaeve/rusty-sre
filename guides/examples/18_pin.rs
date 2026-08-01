//! `Future` is a state machine polled through `Pin<&mut Self>`. `Poll::Pending`
//! promises that a waker will request another poll when progress may be possible.
//!
//! Pinning prevents safe code from moving an address-sensitive value. Most types
//! are `Unpin` and gain no restriction; `PhantomPinned` opts a type out. Pinning
//! does not keep a value alive, schedule a future, or make memory thread-safe.
//!
//! Further reading:
//! - <https://doc.rust-lang.org/std/pin/index.html>
//! - <https://doc.rust-lang.org/std/future/trait.Future.html>
//! - <https://doc.rust-lang.org/std/task/enum.Poll.html>
//! - <https://doc.rust-lang.org/std/task/struct.Waker.html>
//! - <https://doc.rust-lang.org/book/ch17-05-traits-for-async.html>
//! - Source study: <https://github.com/hyperium/hyper/blob/master/src/proto/h1/dispatch.rs>
//! - Source study: <https://github.com/tokio-rs/tokio/tree/master/tokio/src/runtime/task>

use std::{
    future::Future,
    marker::PhantomPinned,
    pin::{Pin, pin},
    task::{Context, Poll, Waker},
};

#[derive(Debug)]
struct Countdown {
    remaining: u8,
}

impl Future for Countdown {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        // `Countdown: Unpin`, so it is safe to recover `&mut Self` and change state.
        let state = self.get_mut();
        if state.remaining == 0 {
            Poll::Ready("connected")
        } else {
            state.remaining -= 1;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[derive(Debug)]
struct PinnedRequest {
    route: String,
    _pinned: PhantomPinned,
}

impl PinnedRequest {
    fn route(self: Pin<&Self>) -> &str {
        &self.get_ref().route
    }
}

fn main() {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);

    // `pin!` pins a local value for the remainder of its scope.
    let mut future = pin!(Countdown { remaining: 2 });
    assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready("connected"));

    // `Box::pin` gives the pointee a stable heap address. Moving the box handle
    // does not move its pointee.
    let request = Box::pin(PinnedRequest {
        route: "/metrics".to_owned(),
        _pinned: PhantomPinned,
    });
    let before = std::ptr::from_ref(request.as_ref().get_ref());
    let moved_handle = request;
    let after = std::ptr::from_ref(moved_handle.as_ref().get_ref());
    assert_eq!(before, after);
    assert_eq!(moved_handle.as_ref().route(), "/metrics");

    // An `Unpin` pointee can be recovered from `Pin` because moving it is sound.
    let movable = Box::pin(Countdown { remaining: 0 });
    let movable: Box<Countdown> = Pin::into_inner(movable);
    assert_eq!(movable.remaining, 0);
}
