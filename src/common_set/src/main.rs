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
    let slots = vec![1,2,3,4,5];
    let mut node = vec![];
    node.push(Node{slots:Some(slots.clone())});
    node.push(Node{slots:Some(slots.clone())});
    let cluster = Cluster{
        nodes:Some(node),
    };
    range_vec(cluster);
}
fn range_vec(cluster :Cluster)  {
    for node in cluster.nodes.unwrap(){
        for slot in node.slots.as_ref().unwrap(){
            println!("{:?}", slot);
            println!("{:?}",node);
        }
    }
}
struct Cluster {
    nodes :Option<Vec<Node>>,
}
#[derive(Debug)]
struct Node {
    slots :Option<Vec<usize>>,

}
