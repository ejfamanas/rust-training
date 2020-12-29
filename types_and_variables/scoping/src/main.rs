fn scope_and_shadowing() {
    let a = 123;
    // also valid, but terrible
    let a = 1234;
    println!("{}", a);
    {
        let b = 456;
        println!("{}", b);
        // still works due to shadowing
        println!("{}", a);
        let a = 999;
        // this will work too
        // OMFG pointers and instances
        println!("{}", a);
    }
    // won't work due to scoping
    // println!("{}", b);
}

fn main() {
    println!("Hello, world!");
    scope_and_shadowing();
}
