fn main() {
    println!("Hello, world!");
    println!("{}", MY_CONST);
    unsafe {
        println!("{}", MY_OTHER_CONST);
    }
    constants();
}

// global constant
const MY_CONST: u8  = 42;
// another way of doing it
// but this will warn because it is potentially unsafe
static MY_OTHER_CONST: i32 = 44;

fn constants() {
    // some stuff from a JS perspective
    {
        const THING_A: char = 'a'; // must declare type, must be in all caps when using this keyword
        let thing_b = THING_A; // implicit immutable
        let mut thing_c: &str = "new thing"; // declared mutable, will error since parent ref is const
        // const is evaluated at compile time, and must declare a type
        // let is enforced at runtime
        // but in order to solve the unique a vs re-declared a problem:
        thing_c = "something else"; // OMFERG Type coercion works
        println!("{}, {}, {}", THING_A, thing_b, thing_c);
    }
    {
        let a1 = "thing";
        let mut a2 = "string";
        let b = &a1; // will invoke all a, or a shared reference
        let c = a2; // will invoke mutable a; also known as borrowing
        a2 = "new thing"; // c will still be the same thing because of primitive typing
        // need to type specify the borrowed variable
        println!("{}, {}", b, c);
    }
}