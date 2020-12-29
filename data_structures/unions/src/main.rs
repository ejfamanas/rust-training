union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat {i: 42} => {
                println!("meaning of life value")
            }
            IntOrFloat {f } => {
                println!("value = {}", f);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let iof = IntOrFloat { i: 42 };
    process_value(iof);
}
