fn main() {
    let v = 10;
    println!("v{}", v);
    let mut m = 10;
    println!("m{}", m);
    m = 11;
    println!("m{}", m);

    // const
    const C_CONST: u32 = 11;
    println!("c{}", C_CONST);

    // shadow
    let _s = 5;
    let _s = 11;
    println!("s should be 11 s{}", _s);
}
