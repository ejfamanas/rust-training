fn if_statement(temp: i32) {
    if temp > 30 {
        println!("it's really hot outside!")
    } else if temp < 10 {
        println!("FARK ITS COLD!!");
    } else {
        println!("temp is just right");
    }
    // similar to a ternary operation but uses declarative keywords instead of symbols
    let day = if temp > 20 {"sunny"}
    else if temp < 20 {"cord!!!"}
    else {"cloudy"};
    println!("{}", day);
    println!(" nested if {}", if temp > 20 {
        if temp > 30 {"very hot"} else {"hot"}
    } else {"okay"})
}

fn main() {
    if_statement(50);
    println!("minished!");
}
