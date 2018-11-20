fn main() {
    let s = Rect::square(10);
    let area = s.area();
    println!("squra {:?} area {}", s, area);
    let mut p = Persion{
        name:String::from("lth"),
        age:25,
        addrs: 
    }
}
#[#[derive(Debug)]]
struct Persion {
name :String ,
age :u16,
addrs:&str,
}
#[derive(Debug)]
struct Rect {
    weight: i64,
    height: i64,
}
impl Rect {
    // add code here
    fn area(&self) -> i64 {
        self.weight * self.height
    }
    fn square(size: i64) -> Rect {
        Rect {
            weight: size,
            height: size,
        }
    }
}
