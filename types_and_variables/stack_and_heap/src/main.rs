#[path = "support/sh.rs"]
mod sh;

fn main() {
    println!("Hello, world!");
    sh::stack_and_heap();
}
