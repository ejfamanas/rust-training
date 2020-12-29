struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point {x: 4.0, y: 5.0};
    let line = Line {start: p, end: p2};
    println!("{}, {}", p.x, p.y);
}


fn main() {
    structures();

    println!("Hello, world!");
}
