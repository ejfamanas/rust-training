// functions that take functions, or return functions, or both
// f(g) {let x = g()}
// functions that return other functions are called generators
// f() -> g() -> x

//ex sum of all even squares where x < 500

// returning a function
// move is to ensure the variable lives long enough
// to call this function, otherwise it gets destroyed
// on function call
// the function also needs a signature of implements the function trait
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}


fn even_squares_sum() {
    let limit: u32 = 500;
    let mut sum: u32 = 0;
    let is_even: fn(u32) -> bool = |x: u32| -> bool { x % 2 == 0 };
    // cannot type this here as impl Trait is not allowed outside of function and
    // inherent method return types
    let above_limit = greater_than(limit);
    for i in 0.. {
        let isq = i * i;

        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);
    // piping
    let sum2 = (0..)
        .map(|x| x * &x) // initial function, but moves value
        .take_while(|&x| x < limit) // need to reference to test the value as it has moved
        .filter(|x| is_even(*x)) // like filter, but you need to dereference
        .fold(0, |sum, x| sum + x); // uses reduce and an accumulator

    println!("sum2 = {}", sum2);
}

fn main() {
    println!("Hello, world!");
    even_squares_sum();
    println!("minished!");
}
