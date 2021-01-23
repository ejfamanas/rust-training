use std::thread;
use std::sync::{Arc, Mutex};

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name, state }
    }

    fn greet(&self) {
        // this will not work because multiple
        // threads are trying to use the same variable
        // we have to use a mutex, or a mutual exclusion
        // so the threads cannot modify the variable simultaneously
        // so we lock them first
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
    }
}

fn arc_demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());
    // here we are moving the object to another thread
    // but the trait needs to be implemented
    let t = thread::spawn(move || {
        person.greet();
    });
    // we need to lock the mutex here
    println!("Name = {}, State = {}", name, state.lock().unwrap().as_str());
    // closing the thread
    t.join().unwrap();
}

fn main() {
    println!("Hello, world!");
    // reference counting
    arc_demo();
}