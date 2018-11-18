fn main() {
    let mut s = String::from("sss");
    println!("s {}", s);
    s.push_str("aaa");
    println!("s {}", s);
    {
        let s = "aa";
        println!("s {}", s);
    }
    let s2 = s;
    // println!("s {}", s); error of move
    let mut s3 = s2.clone();
    println!("s2 {} s3 {}", s2, s3);
    println!("s3 {} len {}", s3, &get_len(&s3));
    mut_borrow(&mut s3);
    println!("mut borrow {}", s3);
}

// borrow
fn get_len(s: &String) -> usize {
    s.len()
}

fn mut_borrow(s: &mut String) {
    s.push_str("mut borrow");
}
