#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

fn main() {
    // `0..5` is like Python3's `range(5)` - it returns an iterator.
    for i in 0..5 {
        // No need for C-like ternary operator, `if-else` is an expression, i.e. it returns a value.
        // Note that the implicit return works only if there are no semicolons.
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("Hello {} {}!", even_odd, i);
    }

    // Inclusive - 5 will be printed as well
    for i in 0..=5 {
        println!("Hello {}!", i);
    }
}
