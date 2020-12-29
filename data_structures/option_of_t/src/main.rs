fn main() {
    println!("Hello, world!");
    let x = 3.0;
    let y = 1.0;

    // Option<T>  -> val | none here we are using it to test if an operation result
    // can test Some and None
    // Some is maintaining the function of x/y and storing it as value z
    let result = if y != 0.0 {Some(x/y)} else {None};

    match result {
        Some(z) => println!("{} / {} = {}",x,y,z),
        None => println!("Cannot divide by 0")
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }
}
