// rust supports static dispatch and dynamic dispatch

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

// modern morphization / polymorphism

// this is a static dispatch because the decision to
// invoke the function at compile time
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

fn main() {
    println!("Hello, world!");
    let a = 123;
    let b = "hello".to_string();
    print_it(a);
    print_it(b);
    println!("minished!");
}
