fn main() {
    println!("Hello, world!");
    // owned by the stack
    // but data is on the heap
    let v = vec![1,2,3];
    // v has moved and can no longer be used anywhere
    let v2 = v;
    let foo = |v:Vec<i32>| ();
    // this will not compile
    foo(v);
    let u = 1;
    let u2 = u;
    // this will work because primitives perform
    // a full copy
    println!("u = {}",u);

    let g = Box::new(1);
    let g2 = v;
    // this will not work, because box is putting
    // the value on the heap
    printlin!("v = {}", v);

    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    // this will work because we are using the value
    // then returning it
    let vv = print_vector(v);
}
