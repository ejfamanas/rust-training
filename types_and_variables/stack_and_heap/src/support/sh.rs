#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {
    // stack allocation
    let p1: Point = origin();
    // heap allocation
    let p2 = Box::new(origin());

    // p1 is looking at the obj value on the stack
    println!("p1, {}", mem::size_of_val(&p1));
    // this will be different, because p2 is looking at a pointer on the heap
    println!("p2, {}", mem::size_of_val(&p2));
    // looking at the val the pointer is pointing to
    println!("p2, {}", mem::size_of_val(&*p2));
    // now we are placing it as a var on the stack
    let p3 = *p2;
    println!("p3, {}", mem::size_of_val(&p3));
}