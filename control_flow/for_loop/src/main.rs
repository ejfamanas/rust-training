fn for_loop() {
    // if we want to include 10, 11 is the stop val, like array length
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        println!("x = {}", x);
    }
    // getting the position in the collection
    // must call the enumerate function in order to get the
    // index
    for (pos, y) in (30..41).enumerate() {
        println!("position = {}, val = {}", pos, y);
    }
}

fn main() {
    println!("Hello, world!");
    for_loop();
    println!("minished!");
}
