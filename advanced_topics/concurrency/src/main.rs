use std::thread;
use std::time;

fn main() {
    println!("Hello, world!");
    // here we are creating a new thread
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            println!("+");
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
    for _ in 1..10 {
        println!("_");
        thread::sleep(time::Duration::from_millis(300));
    }
    // joins the thread to ensure both threads perform
    handle.join();
}
