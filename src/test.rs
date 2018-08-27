extern crate futures;
extern crate rand;

#[cfg(test)]
mod tests {
    use futures::channel::oneshot;
    use futures::channel::mpsc;
    use futures::stream::{Stream, StreamExt};
    use futures::sink::Sink;
    use futures::executor::block_on;
    use futures::future::{self, Future};
    use futures::task::{self, Poll, Context};


    use super::rand;

    use std::thread;
    use std::time::Duration;
    use std::pin::PinMut;

    fn expensive_computation() -> u32 {
        thread::sleep(Duration::from_millis(10));

        200
    }

    #[test]
    fn oneshot_sender_send() {
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            tx.send(expensive_computation()).unwrap();
        });


        let val = block_on(rx).unwrap();
        assert_eq!(val, 200);
    }

    struct PendingOnce {
        first:bool
    }
    impl Future for PendingOnce {
        type Output = ();

        fn poll(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Self::Output> {
            if self.first {
                self.first = false;
                cx.waker().wake();
                Poll::Pending
            } else {
                Poll::Ready(())
            }

        }
    }


}

