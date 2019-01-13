use futures::stream;
use futures::sync::mpsc;
use futures::sync::oneshot::channel;
use futures::*;
use std::{thread, time};

pub fn one_shot_chan() {
    let (t, r) = channel::<i32>();
    let th = thread::spawn(move || {
        r.map(|i| {
            println!("receive {}", i);
        })
        .wait()
        .unwrap();
    });
    t.send(3).unwrap();
    th.join().unwrap();
}

pub fn chan() {
    let (mut tx, rx) = mpsc::channel::<i32>(20);
    // let mut tx2 = tx.clone();

    thread::spawn(move || {
        for i in 0..10 {
            tx.start_send(i).unwrap();
            tx.poll_complete();
        }
        tx.close();
    });

    // let t2 = thread::spawn(move || {
    //     tx2.start_send(2).unwrap();
    // });
    // let rx = rx.map(|x| {
    //     if x % 2 == 0 {
    //         return Some(x);
    //     } else {
    //         None
    //     }
    // });
    // thread::spawn(move || {
    //     thread::sleep(time::Duration::from_secs(5));
    //     for i in 20..30 {
    //         tx.start_send(i).unwrap();
    //     }
    //     tx.poll_complete();
    // });
    tokio::run(MyStream::new(rx));
}

struct MyStream<I>
where
    I: Stream,
{
    input: I,
}
impl<I> MyStream<I>
where
    I: Stream<Item = i32, Error = ()>,
{
    pub fn new(input: I) -> MyStream<I> {
        MyStream { input }
    }
}

impl<I> Future for MyStream<I>
where
    I: Stream<Item = i32, Error = ()>,
{
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Result<Async<()>, Self::Error> {
        loop {
            match try_ready!(self.input.poll()) {
                None => {
                    println!("close");
                    return Ok(Async::Ready(()));
                }
                Some(val) => {
                    println!("val {:?}", val);
                    continue;
                }
            }
        }
    }
}
