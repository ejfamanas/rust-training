struct Person {
    name: String,
}

impl Person {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

// the generic is used to define the lifetime
struct Company<'z> {
    name: String,
    // here we need static to define the lifetime for the ref
    ceo: &'z Person,
}

fn main() {
    println!("Hello, world!");
    // a is a static member, its a lifetime, how long
    // the variable will live. static will live for
    // as long the program will live
    let a: &'static str = "my string";
    let boss = Person {name: String::from("Elon Musk")};
    // this will now work because company has a generic lifeline contract
    // to ensure the ref for Person used by ceo does not die before
    // company
    let tesla = Company {name: String::from("Tesla"), ceo: &boss};
    println!("{}", boss.get_ref_name());
    println!("{}", tesla.ceo.get_ref_name());
    // will also work
    let person = Person {name: String::from("John")};
    println!("{}", person.get_ref_name());
}
