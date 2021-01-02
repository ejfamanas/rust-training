// used to describe a set of unique elements, and the elements are unordered

use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);
    // if a duplicate is inserted, nothing will happen
    greeks.insert("gamma");
    println!("{:?}", greeks);
    // when an insert is performed, a boolean is returned on whether the operation succeeded
    // or not
    let added_vega = greeks.insert("vega");
    println!("{:?}", added_vega); // prints true
    let added_delta = greeks.insert("delta");
    println!("{:}", added_delta); // prints false
    // test contains
    let test_kappa = greeks.contains("kappa");
    println!("{:?}", test_kappa); // prints false
    // removes also returns a boolean based on successful operation or not
    let removed_delta = greeks.remove("delta");
    println!("{:?}", removed_delta); // prints true
    println!("{:?}", greeks);

    // venn operations can be used against hash sets
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // testing subset
    println!(
        "is {:?} a subset of {:?} {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    // testing disjoint = no common elements
    println!(
        "is {:?} disjoined from {:?} {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );
    // testing union, intersection

    println!(
        "is what items are a union of {:?} and {:?} {:?}",
        _2_8,
        _6_10,
        _2_8.union(&_6_10)
    );
    // intersection = difference
}
