use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    let sender = thread::spawn(move || {
        let mss = "a";
        tx.send(mss).unwrap();
        println!("send message");
    });
    let receiver = thread::spawn(move || {
        let v = rx.recv().unwrap();
        println!("receive message {}", v);
    });
    sender.join().unwrap();
    receiver.join().unwrap();
    name();
}

fn name() {
    let n = thread::Builder::new()
        .name("custom thread".into())
        .stack_size(10240)
        .spawn(|| {
            let current = thread::current();
            println!("current thread {:?}", current.name());
        })
        .unwrap();
    n.join().unwrap();
}
