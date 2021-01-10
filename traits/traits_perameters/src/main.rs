use std::fmt::Debug;

struct Circle {
    radius: f64,
}

// deriving debug trait
#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        &self.side * &self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        &self.radius * &self.radius * std::f64::consts::PI
    }
}

// declaring a parameter which inherits from the base class
fn print_info(shape: impl Shape) {
    println!("area of the shape  = {}", shape.area());
}

// multi implementation decoration
fn print_more_info(shape: impl Shape + Debug) {
    println!("line printed using debug = {:?}", shape);
}

// using trait debounce
fn print_using_debouncing<T: Shape + Debug>(shape: T, shape2: T) {
    println!("{}, {}", shape.area(), shape2.area());
}

// using trait with the where keyword
fn print_using_where<T>(shape: T) where T: Shape + Debug {
    println!("{:?}", shape);
}

fn main() {
    println!("Hello, world!");
    let c = Circle { radius: 23.7 };
    let s = Square { side: 22.7 };
    print_info(c);
    print_more_info(s);
    let x = Square { side: 10.1 };
    let y = Square { side: 10.2 };
    print_using_debouncing(x, y);
    let z = Square{ side: 5.24};
    print_using_where(z);
    println!("Minished!");
}
