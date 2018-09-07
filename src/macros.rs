use core::pin::PinMut;
use futures::future::Future;
use futures::task::{self, Poll};

#[macro_export]
macro_rules! yield_now {
    () => {
        await!($crate::macros::pending_once())
    }
}

#[doc(hidden)]
pub fn pending_once() -> PendingOnce {
    PendingOnce { is_ready: false }
}

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct PendingOnce {
    is_ready: bool,
}

impl Future for PendingOnce {
    type Output = ();
    fn poll(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        if self.is_ready {
            Poll::Ready(())
        } else {
            self.is_ready = true;
            cx.waker().wake();
            Poll::Pending
        }
    }
}
