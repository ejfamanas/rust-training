// arrays vs slices
// arrays are of fixed size
// slices are of unknown size

// here we use the &, the borrowing symbol
fn use_slice(slice: &mut [i32]) {
    println!("first element of the slice = {}, length is = {}", slice[0], slice.len());
    println!("printing the whole array = {:?}", slice);
}

fn slices() {
    let mut data = [1,2,3,4,5];

    // takes a segment of the array
    use_slice(&mut data[1..4]);

    // takes the whole array
    use_slice(&mut data);
}

fn main() {
    println!("Hello, world!");
    slices();
}
