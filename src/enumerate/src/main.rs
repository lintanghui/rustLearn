fn main() {
    let method = Method::DELETE;
    if let Method::DELETE = method {
        println!("method delete");
    }

    match method {
        Method::DELETE => println!("delete"),
        Method::GET => println!("get"),
        Method::HEAD => println!("head"),
        Method::PUT => println!("put"),
        Method::OPTIONS => println!("OPTIONS"),
        _ => println!("other",),
    }
    let method = Method::GET;
    println!("get ispower {}", method.is_power());
}
#[derive(Debug)]
enum Method {
    POST,
    GET,
    DELETE,
    HEAD,
    PUT,
    OPTIONS,
    PATCH,
}

impl Method {
    fn is_power(self) -> bool {
        match self {
            Method::POST | Method::PATCH => false,
            _ => true,
        }
    }
}
