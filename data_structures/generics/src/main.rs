struct Point<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

fn generics() {
    let a: Point<u8> = Point {x: 1, y: 10};
    let b: Point<f32> = Point {x: 1.1, y: 1.2};
    println!("{}, {}, {}, {}", a.x, a.y, b.x, b.y);
}

fn main() {
    println!("Hello, world!");
    generics();
}
