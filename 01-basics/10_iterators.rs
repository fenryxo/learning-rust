#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

fn main() {
    // Iterator has next() returning an Option (Some/None)
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    let array = [10, 20, 30, 40];
    // Cannot directly iterate over array:
    /*
        for item in array {
            `[{integer}; 4]` is not an iterator; maybe try calling `.iter()` or a similar method
    */
    // Must use .iter()
    for item in array.iter() {
        println!("Array item: {}", item);
    }

    let v = vec!(10, 20, 30, 40);
    // You can directly iterate over vector, but items are moved (consumed).
    for item in v {
        println!("Vector item: {}", item);
    }

    let mut v = vec!(10, 20, 30, 40);

    // You can iterate over slices directly without moving/consuming
    for item in &array {
        println!("Array slice item: {}", item);
    }
    for item in &v {
        println!("Vector slice item: {}", item);
    }

    // Idiomatic sum
    // Rust needs type.
    let sum: i32 = array.iter().sum();
    println!("Array sum: {}", sum);
    // Both `v.sum()` & `(&v).sum()` don't work.
    let sum: i32 = v.iter().sum();
    println!("Vector sum: {}", sum);

    // You can extend vector with a compatible iterator
    v.extend(1..20);
    println!("Extended Vector: {:?}", v);
}

