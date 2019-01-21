use futures::task;
use futures::{Async, Future, Poll};
use futures::{Sink, Stream};
use std::fmt::Debug;
use tokio::io::copy;
use tokio::net::tcp::{ConnectFuture, TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::runtime::current_thread;
use tokio_codec::{Decoder, Encoder};
use self::codec::Error;
mod codec;
mod stdnet;

struct Connect {
    conn: ConnectFuture,
}

// impl Future for Connect {
//     type Item = ();
//     type Error = ();
//     fn poll(&mut self) -> Poll<(), Self::Error> {
//         match self.conn.poll() {
//             Ok(Async::Ready(socket)) => {
//                 self.conn.;
//                 Ok(Async::Ready(())
//             }
//             Ok(Async::NotReady) => Ok(Async::NotReady),
//             Err(e) => {
//                 println!("conn err {}", e);
//                 Ok(Async::Ready(()))
//             }
//         }
//     }
// }

pub fn server(addr: &str) {
    let host = addr.parse().unwrap();
    let listener = TcpListener::bind(&host).expect("bind addr fail");
    let server = listener
        .incoming()
        .map_err(|err| eprintln!("get connect err {:}", err))
        .for_each(move |sock| {
            let (tx, rx) = codec::Codec.framed(sock).split();
            let handle = Handle::new(rx, tx).map_err(|err| println!("err"));
            tokio::spawn(handle);
            Ok(())
        });
    tokio::run(server);
}

struct Handle<I, O>
where
    I: Stream<Item = codec::Cmd>,
    O: Sink<SinkItem = codec::Cmd>,
{
    stream: I,
    sink: O,
}

impl<I, O> Handle<I, O>
where
    I: Stream<Item = codec::Cmd>,
    O: Sink<SinkItem = codec::Cmd>,
{
    fn new(stream: I, sink: O) -> Handle<I, O> {
        Handle { stream, sink }
    }
}

impl<I, O> Future for Handle<I, O>
where
    I: Stream<Item = codec::Cmd>,
    O: Sink<SinkItem = codec::Cmd>,
{
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        loop {
            match self
                .stream
                .poll()
                .map_err(|err| println!("fail to poll from upstream "))?
            {
                Async::Ready(Some(item)) => {
                    let cmd: codec::Cmd = Into::into(item);
                    println!("item{:?}", cmd);
                    self.sink
                        .start_send(cmd)
                        .map_err(|err| println!("send err"));
                    self.sink
                        .poll_complete()
                        .map_err(|err| println!("flush err"))?;
                }
                Async::Ready(None) => {
                    println!("none  ");
                    return Ok(Async::Ready(()));
                }
                Async::NotReady => {
                    return Ok(Async::NotReady);
                }
            }
        }
    }
}
