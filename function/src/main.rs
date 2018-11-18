fn main() {
    let a = 1;
    let b = 32.2;
    with_params(a, b);
    let c = 2;
    let v = with_return(a, c);
    println!("return {}", v);
    let sub = with_return(11, 2);
    let i = if sub > 10 { 1 } else { 2 };
    println!("sub {} letif {}", sub, i);
    // fib
    println!("fib {} {} {} {} {}", fib(1), fib(2), fib(3), fib(4), fib(5));
}

fn with_params(a: i32, b: f64) {
    println!("a {} b {}", a, b);
}

fn with_return(a: i32, b: i32) -> i32 {
    if a < 10 {
        a + b
    } else {
        a - b
    }
}

fn fib(n: i32) -> i32 {
    let mut first = 1;
    let mut second = 1;
    let mut result: i32 = 1;
    let mut idx = 2;
    while idx < n {
        result = first + second;
        first = second;
        second = result;
        idx = idx + 1;
    }
    result
}
