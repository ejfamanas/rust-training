fn print_value(x: i32) {
    println!("value = {}", x);
}

// here we are using a function to manipulate
// a mutable reference but not providing a return value
// the value is currently on the heap
fn increase(x: &mut i32) {
    // need to dereference using the * symbol to manipulate the
    // value itself
    *x += 1;
}

// the function will take two numbers
// and return the product of the two numbers
fn prod(x: i32, y: i32) -> i32 {
    // no semicolon means implicit return
    x * y
}

fn functions() {
    // this function will go on the stack
    print_value(33);

    let mut z = 1;
    // need to put mut before to pass it as a mutable
    // otherwise it will be passed as an immutable reference
    // by default
    increase(&mut z);
    println!("z = {}", z);
    println!("product = {}", prod(2,3));
}

fn main() {
    println!("Hello, world!");
    functions();
    println!("minished!");
}
