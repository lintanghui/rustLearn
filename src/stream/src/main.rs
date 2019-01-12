use futures::prelude::*;
use futures::stream;
use futures::sync::mpsc;
use futures::try_ready;

use std::thread;
fn main() {
    let (mut tx, rx) = mpsc::channel::<i32>(20);

    tx.start_send(1);
    tx.start_send(2);
    tx.poll_complete();
    let rx = rx
        .map(|x| -> i32 {
            println!("x {}", x);
            x + 3
        })
        .map(|x| println!("xx {}", x));
    tokio::run(myStream::new(rx));
}

struct myStream<I>
where
    I: Stream,
{
    input: I,
}
impl<I> myStream<I>
where
    I: Stream<Item = (), Error = ()>,
{
    pub fn new(input: I) -> myStream<I> {
        myStream { input }
    }
}

impl<I> Future for myStream<I>
where
    I: Stream<Item = (), Error = ()>,
{
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        loop {
            match try_ready!(self.input.poll()) {
                Some(val) => {
                    println!("val {:?}", val);
                    continue;
                }
                None => {
                    return Ok(Async::Ready(()));
                }
            }
        }
    }
}
