use std::rc::Rc;

struct Person {
    name: Rc<String>
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person {name}
    }

    fn greet(&self) {
        println!("Hi, my name is {}", &self.name)
    }
}
fn rc_demo() {
    let name = Rc::new("John".to_string());
    // getting the strong pointers using rc
    println!("Name = {} and has {} strong pointers", name, Rc::strong_count(&name));
    // in this method, name is no longer iin scope
    // clone now needs to be called
    // clone increments the RC by 1
    let person = Person::new(name.clone());
    person.greet();
    // by adding rc decorator above
    // name can now be used
    println!("Name = {}", name);
    println!("Name = {} and has {} strong pointers", name, Rc::strong_count(&name));
}

fn main() {
    println!("Hello, world!");
    // reference counting
    rc_demo();
}
