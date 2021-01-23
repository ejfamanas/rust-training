fn main() {
    println!("Hello, world!");
    // in this reference, the reference is immutable
    let print_vector = |x:&Vec<i32>| -> () {
        println!("{:?}", x);
    };
    let v = vec![3,2,1];
    // instead of letting the function take ownership
    // use the reference
    // this will still work and v can be used later on
    print_vector(&v);
    println!("{:?}", v);

    // mutable references
    let mut a = 40;
    let b = &mut a;
    // the star character lets you access the memory
    // where the original mutable reference was located
    *b += 2;
    // we can no longer use a at this point
    // but...
    let mut a = 40;
    // this will also work
    // but you need to declare the mut ref
    let &mut c = &a;
    {
        // this will also work because its scoped
        let b = &mut a;
        *b += 2;
    }
    // a can still be used
    println!("{}", a);
    // this will work because of the ref
    for i in &z {
        println!("i = {}", i);
        // but we cannot do this
        z.push(5);
    }
}
