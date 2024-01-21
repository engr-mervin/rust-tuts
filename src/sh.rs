use std::mem;

struct Point{
    x: f64,
    y: f64
}

fn origin() -> Point{
    Point{x:0.0,y:0.0}
}

pub fn stack_and_heap(){
    let p1:Point = origin(); //allocate in stack, p1 refers to the actual value
    let p2: Box<Point> = Box::new(origin()); //allocate in heap, p1 refers to the pointer to the value


    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); //actual value is 2 * f64 = 16 bytes
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); //reference is isize = 8 bytes


    let p3: Point = *p2;   
    println!("{}", p3.x);
}
