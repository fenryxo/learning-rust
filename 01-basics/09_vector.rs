#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

fn main() {
    // Vectors must be mutable
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    // Same as for array
    let first = v[0];
    let fifth = v.get(4);
    println!("Vector: {:?}", v);
    println!("First: {:?}", first);
    println!("Fifth: {:?}", fifth);

    // We can slice vector
    let slice = &v[1..4];
    println!("Slice: {:?}", slice);

    // Vector macro
    let mut v = vec![1, 10, 5, 1, 2, 11, 2, 40];
    println!("Original v: {:?}", v);
    // Sorting
    v.sort();
    println!("Dorted v: {:?}", v);
    // Deduplication
    v.dedup();
    println!("Deduplicated v: {:?}", v);
    // Pop
    println!("Popped value: {:?}", v.pop());
    println!("After v: {:?}", v);
    // Insert
    v.insert(3, 999);
    println!("Final v: {:?}", v);
    // Clone
    let mut v2 = v.clone();
    println!("Clone v2: {:?}", v2);
    v2.clear();
    println!("Empty v2: {:?}", v2);
}

