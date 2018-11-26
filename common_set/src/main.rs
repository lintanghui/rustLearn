#![feature(vec_remove_item)]
fn main() {
    // vec
    let mut v = Vec::new();
    v.push(1);
    v.push(-1);
    println!(" pop1 {}", v.pop().unwrap());
    let mut v = vec![1, 2, 3, 4];
    for i in &v {
        println!("{}", i);
    }
    let first = &v[0];
    println!("{}", first);
    v.push(5);
    v.remove(2);
    let rm = 4;
    v.remove_item(&rm);
    println!("{:?}", v);
    
}
