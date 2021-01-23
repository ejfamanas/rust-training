struct Person<'z> {
    name: &'z str
}

// we need to implement the lifetime generic here
impl<'z> Person<'z> {
    fn talk(&self) {
        println!("Hi, my name is {}", &self.name)
    }
}

fn main() {
    println!("Hello, world!");
    let person = Person {name: "John"};
    println!("{}", person.name);
    person.talk();
}
