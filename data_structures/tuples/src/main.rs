fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (&x + &y, &x * &y)
}

fn tuples() {
    let x: i32 = 3;
    let y: i32 = 4;
    let sp: (i32, i32) = sum_and_product(x, y);
    println!("{:?}", sp);

    // destructuring
    let (a, b) = sp;
    println!("first pos = {}, second pos = {}", a, b);
    let sp2 = sum_and_product(4, 7);
    let combined = (&sp, sp2);
    println!("{:?}", combined);
    println!("last element of tuple = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;
    println!("{},{},{},{}", c, d, e, f);
    // single element tupple
    let single = (42,);
    println("{}", single);
}

fn main() {
    println!("Hello, world!");
    tuples();
    println!("minished!");
}
