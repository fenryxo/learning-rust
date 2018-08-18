#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

fn main() {
    // Type is `[i32, 4]`, i.e. the array size is included in te type - compile time checks.
    let array = [10, 20, 30, 40];  // Not a Python list!
    let first = array[0];
    println!("Length {}", array.len());  // `.len()`, not `.length()` or `.length`
    println!("First {}", first);

    for i in 1..array.len(){
        println!("[{}] is {}", i, array[i]);
    }

    // Need to use `{:?}` for arrays (debug print).
    println!("Sum of {:?} is {}", array, sum(&array));  // We pass a slice &[i32]
    // Equivalent to Python's [1:3], [1:], and [:3]
    println!("Some slices: {:?}, {:?}, {:?}.", &array[1..3], &array[1..], &array[..3]);

    // .get() returns Some/None
    let slice = &array;
    let first = slice.get(0);
    let fifth = slice.get(4);
    println!("First: {:?}, is Some: {}, is None: {}", first, first.is_some(), first.is_none());
    println!("Fifth: {:?}, is Some: {}, is None: {}", fifth, fifth.is_some(), fifth.is_none());

    // Safe unwrap with fallback value:
    let fifth_value = * fifth.unwrap_or(&-1);
    println!("Fifth: {}", fifth_value);
}


// We take a slice `&[i32]`
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..numbers.len() {
        result += numbers[i];
    }
    return result;
}
