fn vectors() {
    // vectors will grow dynamically, unlike arrays
    // need to declare mut in order to use the push and pop functions
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    a.push(4);
    // usize, isize variables which will drive the memory allocation, signed vs unsigned integer
    // this will be driven by the operating system and the processor of the host machine
    let idx:usize = 0;
    println!("a[0] = {}", a[idx]);

    // vectors, the size is unknown
    // this will return an option type
    // use some or none to test value if undefined / null
    match a.get(6) {
        Some(x) => println!("a @ 6 = {}", x),
        None => println!("element does not exist")
    }

    for x in &a {
        println!("{}", x);
    }
    // pop also returns an Option
    // {:?} is the debugger placeholder
    println!("final element = {:?}", a.pop());

    // here we are making a let binding to x, and if x != None, run the printlin
    // None will break the loop
    while let Some(x) = a.pop() {
        println!("{}", x)
    }
}

fn main() {
    println!("Hello, world!");
    vectors();
    println!("minished");
}
