fn main() {
    println!("Hello, world!");
    let name = "My string";
    // the format macro will put it into a string format so it can be passed
    // as an arg to another macro function
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);
    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forrest = "forrest";
    // here we are using positional arguments using the format macro
    let rfr = format!("{0}, {1}, {0}", run, forrest);
    println!("{}", rfr);

    // here we are naming the variables and assigning them in the function
    let info = format!(
        "The name's {last}. {first} {last}",
        first = "James",
        last = "Bond"
    );
    println!("{}", info);

    // we are mixing by adding in index and adding in order
    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha", // index 0
        "beta", // index 1
        data = "delta"
    );
    println!("{}", mixed);
}
