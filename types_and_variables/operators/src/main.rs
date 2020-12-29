
fn main() {
    println!("Hello, world!");
    operators();
}

fn operators() {
    // arithmetic
    let mut a = 2+3*4;
    println!("{}",a);
    a -= 2;
    println!("{}",a);
    // can do -=, +=, *=, /=, %= n
    // a %= 3;
    // println!("{}",a);
    let  a_cubed = i32::pow(a, 3);
    println!("{}", a_cubed);
    let b = 2.5;
    // using 64 bit floating point data type, but invoking the integer function where n is an int
    // type b is default to f64
    let b_cubed: f64 = f64::powi(b, 3);
    // here we are invoking powf, where n is a floating point,
    // here we are invoking standard floating point 64 bit float type from constants
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{}, {}", b_cubed, b_to_pi);
    // bitwise operators
    // | OR, & AND ^ XOR ! NOR
    let c: u32 = 1 | 2;
    println!("{}", c);
    // shifting
    let two_ten: u32 = 1 << 10;
    println!("{}", two_ten);
    // logical operators
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("{}", pi_less_4);
    println!("{}", pi_less_4 == true);
}
