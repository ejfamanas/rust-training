struct Person {
    name: String
}

struct UsingIntoPerson {
    name: String
}

struct ScopedGenericPerson {
    name: String
}

impl Person {
    fn new(name: &str) -> Person {
        Person { name: name.to_string() }
    }
}


impl UsingIntoPerson {
    // S must be able to convert into a string
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

impl ScopedGenericPerson {
    fn new<S>(name: S) -> Person where S: Into<String> {
        Person { name: name.into() }
    }
}

fn main() {
    println!("Hello, world!");
    // this is ok
    let john = Person::new("John");
    println!("{}", john.name);
    // using the into as a generic type conversion
    let jane = UsingIntoPerson::new("Jane");
    println!("{}", jane.name);
    // using where
    let joe = ScopedGenericPerson::new("Joe");
    println!("{}", joe.name);
    println!("Minished!");
}
