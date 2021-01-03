// a function that is attached to a struct is a method

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    // an implementation and the method inside
    // will need to access self as a reference
    // in order to access the instantiated values
    fn len(&self) -> f64 {
        let dx = &self.start.x - &self.end.x;
        let dy = &self.start.y - &self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn methods() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let my_line = Line { start: p, end: p2 };
    println!("length = {}", my_line.len());
}

fn main() {
    println!("Hello, world!");
    methods();
}
