pub fn use_box() {
    let mut l = List::new(1);
    l = l.prepend(2);
    l = l.prepend(3);
    assert_eq!(l.len(), 3);
    let b = Box::new(5);
    println!("box {}", b);
    let ptr = Box::into_raw(b);
    unsafe {
        if let Some(b_back) = ptr.as_ref() {
            println!("get value back from box {}", b_back);
        }
    }
}
#[test]
fn test_list() {
    let mut l = List::new(1);
    l = l.prepend(2);
    l = l.prepend(3);
    assert_eq!(l.len(), 3);
}
#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Nil,
}

impl List {
    fn new(i: i32) -> List {
        List::Node(i, Box::new(List::Nil))
    }
    fn prepend(self, i: i32) -> List {
        List::Node(i, Box::new(self))
    }
    fn len(&self) -> i32 {
        match *self {
            List::Node(_, ref sub) => 1 + sub.len(),
            List::Nil => 0,
        }
    }
}
