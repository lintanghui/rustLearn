#[macro_use]
use futures;
use futures::{Async, Future, Poll};
use std::thread;
use tokio::io;
use tokio::io::copy;
use tokio::net::{tcp::ConnectFuture, *};
use tokio::prelude::*;
mod stdnet;

struct Connect {
    conn: ConnectFuture,
}

impl Future for Connect {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.conn.poll() {
            Ok(Async::Ready(socket)) => {
                println!("connect ok {}", socket.peer_addr().unwrap());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => {
                println!("conn err {}", e);
                Ok(Async::Ready(()))
            }
        }
    }
}

pub fn client(addr: &str) {
    let host = addr.parse().unwrap();
    let conn = TcpStream::connect(&host);
    let con = Connect { conn };
    tokio::run(con);
}

pub fn server(addr: &str) {
    let host = addr.parse().unwrap();
    let listener = TcpListener::bind(&host).expect("bind addr fail");
    let server = listener
        .incoming()
        .map_err(|err| eprintln!("get connect err {:}", err))
        .for_each(|sock| {
            let (reader, writer) = sock.split();
            let byte_copied = copy(reader, writer);
            let handle = byte_copied
                .map(|amt| println!("wrote {:}", amt.0))
                .map_err(|err| eprintln!("io err {:}", err));
            tokio::spawn(handle)
        });
    tokio::run(server);
}
