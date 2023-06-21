use project_motion::*;

fn main() {
    let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
}