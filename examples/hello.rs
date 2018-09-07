#![feature(futures_api,async_await,await_macro)]
extern crate futures;
#[macro_use]
extern crate fahrenheit;

use futures::io::{AsyncWriteExt,AsyncReadExt};
use futures::stream::{StreamExt};

use futures::pending;

async fn busy() {
    for i in 0..3 {
        for j in 0..3 {
            println!("current {} {}", i, j);
            yield_now!();
        }
    }
}

async fn haha() {
    for i in 0..10 {
        println!("haha {}", i);
        yield_now!();
    }
}

async fn run() {
    fahrenheit::spawn(busy());
    fahrenheit::spawn(haha());
}

fn main() {
    fahrenheit::run(run())
}

