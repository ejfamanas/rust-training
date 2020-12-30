fn how_many_of(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        3..=11 => "i have some",
        12 => "i have a dozen",
        _ if (x % 2 == 0) => "i have an even number of",
        _ => "i have loads of"
    }
}

pub fn pattern_matching() {
    for x in 0..22 {
        println!("{}: I have {} oranges", x, how_many_of(x))
    }

    let point = (3, 4);
    match (point) {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (ref x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("x = {}, y = {}", x, y)
    }
}