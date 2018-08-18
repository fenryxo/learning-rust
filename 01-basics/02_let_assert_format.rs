#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

fn main() {
    // Type inference. IntelliJ Rust plugin shows the type.
    let answer = 42;
    // Assertions
    assert_eq!(answer, 42);
    /*
    assert_eq!(answer, 43);
        thread 'main' panicked at 'assertion failed: `(left == right)`
        left: `42`,
        right: `43`', ./let1.rs:8:5
    */
    // Python3-like format strings! https://doc.rust-lang.org/std/fmt/index.html
    println!("Hello world, {}!", answer);
}
