use std::thread;
use tokio::io;
use tokio::io::copy;
use tokio::net::*;
use tokio::prelude::*;
pub fn client() {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:8000").unwrap();
    let mut input = stream.try_clone().unwrap();
    let handler = thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        loop {
            match input.read(&mut buffer) {
                Ok(_n) => {
                    std::io::stdout().write(&buffer).unwrap();
                    std::io::stdout().flush().unwrap();
                }
                Err(error) => eprintln!("err read"),
            }
        }
    });
    let output = &mut stream;
    let mut data = String::new();
    loop {
        std::io::stdin().read_line(&mut data);
        output.write(data.as_bytes()).unwrap();
        output.flush().unwrap();
    }
    // let client = TcpStream::connect(&host)
    //     .map_err(|err| eprintln!("connet err {:}", err))
    //     .and_then(|sock| {

    //         let mut input = String::new();
    //         match std::io::stdin().read_line(&mut input) {
    //             Ok(n) => {
    //                 io::write_all(sock, input);

    //             }
    //             Err(error) => println!("error: {}", error),
    //         }

    //         Ok(())
    //     });
    // tokio::run(client);
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
