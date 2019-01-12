use futures::{Async, Future, Poll};
use tokio::io::copy;
use tokio::net::tcp::{ConnectFuture, TcpListener, TcpStream};
use tokio::prelude::*;
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

pub fn client(addr: &str) {
    let host = addr.parse().unwrap();
    let conn = TcpStream::connect(&host)
        .and_then(|stream| {
            let (rstream, wstream) = stream.split();
            let mut wstream = std::io::BufWriter::new(wstream);
            for _i in 1..10 {
                wstream.write_all("aa".as_bytes())?;
            }
            wstream.flush()?;
            let rstream = std::io::BufReader::new(rstream);
            let mut buf = vec![];
            rstream.read_line(buf);
            Ok(())
        })
        .map_err(|err| {
            println!("err {}", err);
        });
    tokio::run(conn);
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
