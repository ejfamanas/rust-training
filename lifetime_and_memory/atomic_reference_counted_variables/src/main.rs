use std::thread;
use std::sync::Arc;

struct Person {
    name: Arc<String>
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", &self.name)
    }
}

fn arc_demo() {
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());
    // here we are moving the object to another thread
    // but the trait needs to be implemented
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);
    // closing the thread
    t.join().unwrap();
}

fn main() {
    println!("Hello, world!");
    // reference counting
    arc_demo();
}
