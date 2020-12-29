enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // WOW this is cool
    CmykColor{cyan:u8, magenta: u8, yellow: u8, black:u8}
}

fn enums() {
    let color:Color = Color::RgbColor(0,0,0);

    match color {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0) | Color::CmykColor {cyan:0, magenta: 0, yellow: 0, black: 0} => println!("black"),
        _ => println!("another color")
    }
}

fn main() {
    println!("Hello, world!");
    enums();
}
