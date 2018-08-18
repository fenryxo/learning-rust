#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

fn main() {
    // Variables are immutable by default. This one is mutable, underlined in IntelliJ Rust plugin.
    let mut sum = 0.0; // The default floating type seems to be f64.
    for i in 0..100 {
        sum += i as f64;  // Type casting.
    }
    println!("Sum: {}", sum);
}
