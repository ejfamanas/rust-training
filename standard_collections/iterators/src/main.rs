fn main() {
    println!("Hello, world!");
    // creates a new vector using the macro
    let mut vec = vec![1, 2, 3];
    // not having a borrow symbol here will actually move
    // it in memory
    for x in &vec {
        // * will follow the reference now
        println!("{}", *x);
    }
    // you can also go through by immutable ref using a function
    for x in vec.iter() {
        // here the * is not necessary
        println!("{}", *x);
        // this won't work: x += 1;
    }
    // vector must be mutable in order to use this function
    for x in vec.iter_mut() {
        // here the * is not necessary
        *x += 2;
        println!("{}", *x);
        // this won't work: x += 1;
    }
    // doing so in reverse
    // iter returns a monad and can stack
    for x in vec.iter().rev() {
        println!("{}", *x);
    }
    // into iter, a move operation that turns the operation
    // into a bi-value operator

    let mut vec2 = vec![3, 2, 1];
    // let it = vec.into_iter(); // same as extend, the original vec will move and get gc'd
    // here we are concat'ing the first vector with the second vector
    vec2.extend(vec);
    println!("{:?}", vec2);
}
