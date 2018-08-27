#![feature(futures_api,async_await,await_macro)]
extern crate futures;
extern crate fahrenheit;

use std::net::SocketAddr;

use futures::io::{AsyncWriteExt,AsyncReadExt};
use futures::stream::{StreamExt};
use fahrenheit::AsyncTcpStream;
use fahrenheit::AsyncTcpListener;

async fn listen(addr: &str) {
    let addr: SocketAddr = addr.parse().unwrap();
    let listener = AsyncTcpListener::bind(addr).unwrap();
    let mut incoming = listener.incoming();

    while let Some(stream) = await!(incoming.next()) {
        fahrenheit::spawn(process(stream));
    }
}

async fn process(mut stream: AsyncTcpStream) {
    let mut buf = vec![0;1024];
    let n = await!(stream.read(&mut buf)).unwrap();
    println!("hahaha {}", n);
    println!("hahaha {}", String::from_utf8(buf).unwrap());
}


fn main() {
	fahrenheit::run(listen("127.0.0.1:12345"))
}
