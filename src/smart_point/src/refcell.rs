use std::cell::RefCell;

pub fn use_refcell() {
    let c = RefCell::new(10);
    {
        let mut c_mut_b = c.borrow_mut();
        *c_mut_b += 1;
    }
    let c_mut = c.borrow();
    assert_eq!(*c_mut, 11);
    let l = List::new();
    l.add();
    l.add();
    println!("list {:?}", l);
}
#[derive(Debug)]
struct List {
    node: RefCell<Vec<String>>,
}

impl List {
    fn new() -> List {
        List {
            node: RefCell::default(),
        }
    }
    fn add(&self) {
        self.node.borrow_mut().push("a".to_string());
    }
}
