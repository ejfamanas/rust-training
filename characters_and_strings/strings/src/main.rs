#[allow(unused_variables)]
fn strings() {
    // this string is a vector of characters
    // we are referencing the static member
    // these are immutable because we are producing a utf-8 value
    let s: &'static str = "hello there"; // &str = string slice.
    // this will work though, even though you cannot individually index characters
    for c in s.chars() {
        println!("{}", c);
    }
    // getting individual characters is verbose
    if let Some(first_char) = s.chars().nth(0) {
        println!("first char = {}", first_char)
    }

    // String - a heap allocation construct
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        // need to push this as push str otherwise will type as static str and wont
        // go in properly
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);
    // converting from &str <> String
    let u: &str = &letters; // uses deref conversion

    // concatenation
    // String + str
    let z = letters + "abc";
    println!("{}", z);
    // converting string slice to string
    let mut abc = String::from("hello world");
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc);
    println!("{}", abc.replace("ello", "goodbye"));
}

fn main() {
    println!("Hello, world!");
    strings();
}
