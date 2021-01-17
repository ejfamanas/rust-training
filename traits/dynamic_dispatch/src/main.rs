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
// simplification
// same implementation, different process
// uses dynamic dispatch, looks at format function based on type at runtime
fn print_it_too(z: &Printable) {
    println!("{}", z.format());
}



fn main() {
    println!("Hello, world!");
    let a = 123;
    let b = "hello".to_string();
    // static dispatch method
    // print_it(a);
    // print_it(b);
    // using dynamic implementation, you must use ref / pointer
    // but this is more expensive
    print_it_too(&a);
    print_it_too(&b);
    println!("minished!");
}
