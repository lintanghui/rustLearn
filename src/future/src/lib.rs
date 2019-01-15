use futures::executor::Notify;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use futures::sync::oneshot::channel;
use futures::*;
use futures::{task, task::Task};
use std::{thread, time::Duration};
use tokio::timer::Timeout;

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
#[derive(Debug)]
struct req {
    num: i32,
    task: Task,
}
pub fn chan() {
    let (mut tx, rx) = mpsc::channel::<req>(20);
    let (tx2, mut rx2) = mpsc::channel::<req>(20);
    // let mut tx2 = tx.clone();

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
    let s = MyStream2::new(rx, tx2.clone());
    let s2 = MyStream::new(rx2, tx.clone()).join(s).and_then(|_| Ok(()));
    // let fut = lazy(move || -> Result<Async<()>, ()> {
    //     for i in 0..2 {
    //         tx.start_send(req {
    //             num: i,
    //             task: task::current(),
    //         })
    //         .unwrap();
    //         // thread::sleep(Duration::from_secs(1));
    //     }
    //     println!("poll");
    //     loop {
    //         match rx2.poll() {
    //             Err(_) => {
    //                 println!("close");
    //             }
    //             Ok(Async::Ready(None)) => {
    //                 println!("note");
    //             }
    //             Ok(Async::Ready(Some(val))) => {
    //                 val.task.notify();
    //                 break;
    //             }
    //             Ok(Async::NotReady) => {
    //                 println!("not ready");
    //                 return Ok(Async::NotReady);
    //             }
    //         }
    //     }
    //     Ok(Async::Ready(()))
    // })
    // .join(s)
    // .and_then(|_| Ok(()));
    // tokio::spawn(fut);
    tokio::run(s2);
}

struct MyStream<I>
where
    I: Stream<Item = req, Error = ()>,
{
    input: I,
    sink: mpsc::Sender<req>,
    task: Option<Task>,
}
struct MyStream2<I>
where
    I: Stream<Item = req, Error = ()>,
{
    input: I,
    sink: mpsc::Sender<req>,
    task: Option<Task>,
}
impl<I> MyStream<I>
where
    I: Stream<Item = req, Error = ()>,
{
    pub fn new(input: I, out: mpsc::Sender<req>) -> MyStream<I> {
        MyStream {
            input,
            sink: out,
            task: None,
        }
    }
    fn add_task(&mut self, task: Task) {
        self.task = Some(task);
    }
}

impl<I> MyStream2<I>
where
    I: Stream<Item = req, Error = ()>,
{
    pub fn new(input: I, out: mpsc::Sender<req>) -> MyStream2<I> {
        MyStream2 {
            input,
            sink: out,
            task: None,
        }
    }
    fn add_task(&mut self, task: Task) {
        self.task = Some(task);
    }
}
impl<I> Future for MyStream<I>
where
    I: Stream<Item = req, Error = ()>,
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
                    val.task.notify();
                }
            }
            self.sink
                .start_send(req {
                    num: 0,
                    task: task::current(),
                })
                .expect("send err");
            println!("task ");
        }
    }
}
impl<I> Future for MyStream2<I>
where
    I: Stream<Item = req, Error = ()>,
{
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Result<Async<()>, Self::Error> {
        loop {
            self.sink
                .start_send(req {
                    num: 1,
                    task: task::current(),
                })
                .expect("send err");
            println!("task ");
            match try_ready!(self.input.poll()) {
                None => {
                    println!("close");
                    return Ok(Async::Ready(()));
                }
                Some(val) => {
                    println!("val {:?}", val);
                    val.task.notify();
                }
            }
        }
    }
}
