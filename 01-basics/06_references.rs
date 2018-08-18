#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

// Reference types have `&` prefix
fn add_immutable(x: &i32) -> i32 {
    // Dereference operator is `*`.
    *x + 1
}


// Reference can be mutable - `&mut` prefix
fn add_mutable(x: &mut i32) {
    // Dereference operator is `*`.
    *x += 1;
}

fn main() {
    for i in 0..5 {
        // Values are passed by reference with `&` operator.
        println!("{} + 1 is {}.", i, add_immutable(&i));

        // A mutable copy.
        let mut i2 = i;
        // Values are passed by mutable reference with `&mut` operator.
        add_mutable(&mut i2);
        assert_eq!(i2, i + 1);
    }
}
