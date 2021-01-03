fn say_hello() {
    println!("hello");
}

fn closures() {
    let sh: fn() = say_hello;
    sh();

    // note the function syntax
    let plus_one: fn(i32) -> i32 = |x: i32| -> i32 {
        x + 1
    };
    println!("using plus_one function = {}", plus_one(7));

    let mut two = 2;
    {
        let plus_two = |_x| {
            let mut z = two;
            z += 2;
            z
        };
        println!("using plus_two function = {}", plus_two(3));
    }
    // this won't work because its been used above
    // the closure needs t release two or make sure the function
    // gets destroyed first, so you scope the above function first
    let borrow_two = &mut two;
    println!("borrwed_two = {}", borrow_two);

    // here we are manipulating the mutable value from inside the
    // closure
    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn main() {
    println!("Hello, world!");
    closures();
    println!("minished!");
}
