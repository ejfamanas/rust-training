use std::collections::HashMap;

// a hashmap is a container for pairs of values

fn main() {
    println!("Hello, world!");

    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes {
        println!(" a {} has {} sides", key, value);
    }

    shapes.insert(String::from("square"), 5);
    println!("a square has {} sides", shapes["square".into()]);

    // test the key value return like get, if not insert val
    shapes.entry(String::from("circle")).or_insert(1);
    println!("a circle has {} side", shapes["circle".into()]);

    // here by doing this, we are mutating the circles value in
    // the hash map because of referencing
    let actual = shapes.entry(String::from("circle")).or_insert(2);
    *actual = 0;
    println!("{:?}", shapes);
}
