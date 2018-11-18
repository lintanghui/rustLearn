fn main() {
    // int
    let a: i8 = 8;
    let b: i16 = 16;
    let c: i32 = 32;
    let d: i64 = 64;
    println!("a {} b {} c {} d {}", a, b, c, d);
    // float
    let f1: f32 = 32.0;
    let f2: f64 = 64.1;
    //  let f3 = f2 - f1; error
    println!("f1 {} f2 {} ", f1, f2);
    // char
    let c1 = 'z';
    let c2 = '@';
    let emoje = '😻';
    println!("emoje {} c1 {} c2 {}", emoje, c1, c2);

    // tuple
    let tup: (i32, i64, f32) = (1, 2, 2.2);
    let (a, b, c) = tup;
    println!("a {} b {} c {}", a, b, c);
    println!("tup {} {} {}", tup.0, tup.1, tup.2);

    //array
    let array = ['1', '2', '3'];
    println!("array {} {}", array[0], array[2]);
}