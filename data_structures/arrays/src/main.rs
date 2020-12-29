use std::mem;

fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("length = {}", a.len());
    println!("first element = {}", a[0]);
    for val in &a {
        println!("{}", val);
    }
    // here we are typing the first value to reduce the size
    let b = [1u16; 10];
    for i in 0..b.len() {
        println!("{}", mem::size_of_val(&b[i]));
    }

    // multidimensional array
    let mtx: [[f32; 3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    // nested loop w/in loop iteration, only identifying diagonal values
    for j in 0..mtx.len() {
        for k in 0..mtx[j].len() {
            if j == k {
                println!("diagonal = {}", mtx[j][k]);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    arrays();
}
