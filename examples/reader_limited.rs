#![feature(futures_api,async_await,await_macro, pin, arbitrary_self_types)]
extern crate futures;
extern crate fahrenheit;
extern crate rand;

use futures::channel::oneshot;
use futures::channel::mpsc;
use futures::stream::{Stream, StreamExt};
use futures::sink::Sink;
use futures::executor::block_on;
use futures::future::{self, Future};
use futures::task::{self, Poll, Context};

use std::thread;
use std::time::Duration;
use std::pin::PinMut;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use futures::task::AtomicWaker;
use futures::executor::LocalPool;
use futures::prelude::SpawnExt;

fn main() {
    let rl = Arc::new(ReadLimited{waker:AtomicWaker::new(), limited: AtomicBool::new(false)});
    let (tx, rx) = mpsc::unbounded();
    let reader = Reader {
        chan:tx,
        limited:rl.clone(),
        index: 0,
    };
    let writer = Writer {
        chan: rx,
        chan_closed: false,
        limited: rl,
        count : 0,
        limit_count:2,
    };

    let mut pool = LocalPool::new();
    let mut spawner = pool.spawner();

    spawner.spawn(reader);
    spawner.spawn(writer);

    pool.run(&mut spawner);
}

struct ReadLimited {
    waker : AtomicWaker,
    limited: AtomicBool,
}

struct Reader {
    chan : mpsc::UnboundedSender<u32>,
    limited: Arc<ReadLimited>,
    index : u32,
}

impl Future for Reader {
    type Output = ();

    fn poll(mut self : PinMut<Self>, cx: & mut Context) -> Poll<Self::Output> {
        self.limited.waker.register(cx.waker());
        if self.limited.limited.load(Ordering::SeqCst) {
            println!("reader: islimited");
            Poll::Pending
        } else {
            println!("reader: send {}", self.index);
            self.chan.unbounded_send(self.index).unwrap();
            self.index += 1;
            if self.index == 100000 {
                println!("reader: send all done");
                Poll::Ready(())
            } else {
                cx.waker().wake();
                Poll::Pending
            }
        }
    }
}

struct Writer {
    chan: mpsc::UnboundedReceiver<u32>,
    chan_closed: bool,
    limited: Arc<ReadLimited>,
    count : u32,
    limit_count: u32,
}

impl Future for Writer {
    type Output = ();

    fn poll(mut self: PinMut<Self>, cx: & mut Context) -> Poll<Self::Output> {
        let have_limited = self.count > self.limit_count;

        println!("writer: have limited is {}, self.count={}", have_limited, self.count);

        let mut ss: &mut Self = &mut (*self);

        if ss.chan_closed == false {
            loop {
                let chan = &mut ss.chan;
                let count = &mut ss.count;
                let chan_closed = &mut ss.chan_closed;

                match chan.poll_next_unpin(cx) {
                    Poll::Ready(Some(_)) => {
                        *count += 1;
                    },
                    Poll::Ready(None) => {
                        *chan_closed = true;
                        println!("writer: chan closed");
                        break;
                    },
                    Poll::Pending =>{
                        break;
                    },
                }
            }
        }

        while ss.count != 0 && ss.poll_write(cx) {
            println!("writer: write ");
            ss.count -= 1;
        }

        if ss.chan_closed && ss.count == 0 {
            println!("writer: done");
            Poll::Ready(())
        } else {
            if have_limited && ss.count <= ss.limit_count {
                println!("writer: wake reader");
                ss.limited.limited.store(false, Ordering::SeqCst);
                ss.limited.waker.wake();
            } else if have_limited == false && ss.count > ss.limit_count {
                ss.limited.limited.store(true, Ordering::SeqCst);
            }
            Poll::Pending
        }
    }

}

impl Writer {
    fn poll_write(&mut self, cx: &mut Context) -> bool {
        if rand::random::<u32>()%4 ==0 {
            true
        } else {
            cx.waker().wake();
            false
        }
    }
}

