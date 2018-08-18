#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

// No type inference for function definitions. That's good, that would be too much magic.
// btw the same syntax as Python type hints ;-)
/// square
fn sqr(x: f64) -> f64 {
    x * x  // Implicit return again.
}


// I really start loving implicit returns and if-else expressions.
/// absolute value of a floating-point number
fn abs(x: f64) -> f64 {
    if x >= 0.0 { x } else { -x }
}


/// ensure the number always falls in the given range
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    assert!(min <= max);
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

// 64bit unsigned int type is u64
/// computes factorial
fn factorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}


fn main() {
    /*
        for i in -0.0..5.0
                 ^^^^^^^^^ the trait `std::iter::Step` is not implemented for `{float}`
    */

    for side in 0..5 {
        // IntelliJ Rust plugin shows names of parameters of a function call, cool!
        let square = sqr(side as f64);
        println!("sqrt({}) is {}.", side, square);
        println!("factorial({}) is {}.", side, factorial(side as u64));
    }

    for i in -5..5 {
        println!("abs({}) is {}", i, abs(i as f64));
        println!("clamp({}, -2, 2) is {}", i, clamp(i as f64, -2.0, 2.0));
    }
}
