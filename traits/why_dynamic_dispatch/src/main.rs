struct Circle { radius: f64 }

struct Square { side: f64 }

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


fn main() {
    println!("Hello, world!");
    let shapes: [&dyn Shape; 4] = [
        &Circle { radius: 1.0 },
        &Square { side: 2.0 },
        &Circle { radius: 3.0 },
        &Square { side: 4.0 }
    ];
    // can only do this using dynamic dispatch
    // because the engine needs to select which area fn implementation
    // to use at runtime
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} jas area {}", i, shape.area());
    }
}
