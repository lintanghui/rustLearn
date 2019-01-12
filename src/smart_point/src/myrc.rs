use std::rc::Rc;

#[derive(Debug)]
enum List {
    Node(i32, Rc<List>),
    Nil,
}
use self::List::{Nil, Node};
pub fn myrc() {
    let r = Rc::new(Node(1, Rc::new(Node(2, Rc::new(Node(3, Rc::new(Nil)))))));
    let a = Rc::new(Node(4, Rc::clone(&r)));
    let b = Rc::new(Node(5, Rc::clone(&r)));
    let _weak = Rc::downgrade(&r);
    println!("a {:?}", a);
    println!("b {:?}", b);
    println!(
        "count a {} b {} r {} weak {} ",
        Rc::strong_count(&a),
        Rc::strong_count(&b),
        Rc::strong_count(&r),
        Rc::weak_count(&r),
    );
}
