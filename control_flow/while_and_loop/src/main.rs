fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 { continue; }
        println!("x = {}", x);
    }
    let mut y = 1;
    loop { // while true, no base case, needs a breaking condition
        y *= 2;
        println!("y == {}", y);

        if y == 1 << 10 { break; }
    }
}

fn main() {
    println!("Hello, world!");
    while_and_loop();
}