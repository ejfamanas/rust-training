#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    // u = unsigned, 8 = 8 bits. this one will be 0-255
    // u = 2^(N-1)
    // the let binding is immutable
    let a: u8 = 123;
    // mut = mutable
    // i8 = signed, -128 - 127
    // i = -2^(N-1)-1
    let mut b: i8 = -128;
    // using substitution
    println!("a = {}, b = {}", a, b);
    // testing out of range
    b = 11;
    println!("b = {}", b);
    // using type inference
    let c = 123456789; //i32
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    // uszie, isize data types, used for processor-specific values
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes", z, size_of_z*8);
    let d: char = 'x'; // char is char, such as punctuation, 32bit unicode character
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));
    // floating point
    // f32, f64, based on IEEE754 floating point standard, these are signed by default
    let e: f32 = 2.5;
    println!("{}, size = {}", e, mem::size_of_val(&e));
    let f: f64 = 2.5; // f64 is the default
    println!("{}, size = {}", f, mem::size_of_val(&f));
    // boolean
    let g: bool = false;
    println!("{}, size = {}", g, mem::size_of_val(&g));
}
