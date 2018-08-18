#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

// Python has `import`, Vala has `using`, Rust has `use`.
use std::f64::consts::PI;

fn main() {
    // IntelliJ Rust plugin shows "Consider using std::f64::consts::PI instead." :-)
    let pi: f64 = 3.1416;
    let x = pi / 2.0;
    let cosine = x.cos();
    println!("cos(pi/2) is {}", cosine);

    // You can redeclare variable.
    let x = 2.0 * PI;
    let cosine = x.cos();
    let abs_difference = (cosine - 1.0).abs();
    assert!(abs_difference < 1e-10);
    println!("cos(2pi) is {}", cosine);

}
