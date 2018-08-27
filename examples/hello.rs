#![feature(futures_api,async_await,await_macro)]
extern crate futures;
extern crate fahrenheit;

use futures::io::{AsyncWriteExt,AsyncReadExt};
use futures::stream::{StreamExt};
use fahrenheit::AsyncTcpStream;
use fahrenheit::AsyncTcpListener;
use std::net::SocketAddr;
use std::io;

async fn listen(addr: &str) {
    let addr: SocketAddr = addr.parse().unwrap();
    let listener = AsyncTcpListener::bind(addr).unwrap();
    let mut incoming = listener.incoming();

    while let Some(stream) = await!(incoming.next()) {
        fahrenheit::spawn(process(stream));
    }
}

async fn process(mut stream: AsyncTcpStream) {
    let mut buf :Vec<u8> = vec![0;1024];
    let name = await!(read_string(&mut stream)).unwrap();
    println!("{}", name);
}

fn main() {
    fahrenheit::run(listen("127.0.0.1:12345"))
}

struct Peer {
    stream: AsyncTcpStream,
    name: String,
}

impl Peer {
    fn new(name:String, stream: AsyncTcpStream) -> Peer {
        Peer { name, stream, }
    }

}

async fn read_u16(stream: &mut AsyncTcpStream) -> Result<u16, io::Error> {
    let mut buf = [0,0];
    await!(stream.read_exact(&mut buf))?;

    Ok(buf[0] as u16 + ((buf[1] as u16) << 8))
}

async fn read_string(stream: &mut AsyncTcpStream) -> Result<String, io::Error> {
    let len = await!(read_u16(stream))?;

    let mut buf = vec![0;len as usize];
    await!(stream.read_exact(&mut buf))?;

    String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

