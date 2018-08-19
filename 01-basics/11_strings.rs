#!../run.sh
// https://stevedonovan.github.io/rust-gentle-intro/1-basics.html

//noinspection SpellCheckingInspection
fn main() {
    // String literals are stored in the binary itself, are immutable and you can get
    // a string slice from them directly.
    let slice = "Hello World";

    // std::string::String is a heap-allocated mutable string buffer.
    // similar to StringBuilder in Vala.
    let mut buffer = String::new();  // mut needed to add a content later

    // You can make a heap-allocated copy of a static string as String.
    let string = slice.to_string();

    // Passing string slices
    dump("Hello World Implicit");
    dump(&"Hello World Explicit");  // works, but unnecessary
    dump(slice);  // slice
    // dump(string);  // implicit not possible
    dump(&string); // coercion to slice

    // Building string
    buffer.push('H');  // append a single character, note single quotes.
    buffer.push_str("ello");  // append string
    buffer += " Dolly!";  // the same
    buffer.pop();  // remove the last character
    dump(&buffer);

    // Everything supported by `println!("{}")` has `to_string()` -> `String`.
    dump(&100.to_string());
    dump(&true.to_string());
    dump(&3.14.to_string());
    // And can be used with `format!()` -> `String`.
    dump(&format!("int {}, bool {}, float {}", 100, true, 3.14));

    // Czech text.
    let utf8 = "Loď čeří kýlem tůň obzvlášť v Grónské úžině.";
    println!("UTF-8 string: '{}'", utf8);
    println!("Length in bytes: {}", utf8.len());
    println!("Length in characters: {}", utf8.chars().count());
    print!("Characters:");
    for c in utf8.chars() {
        print!(" '{}'", c);
    }
    print!("\n");

    // Find a valid index
    let begin = utf8.find('č');
    let end = utf8.find('ť');
    if begin.is_some() && end.is_some() {  // found
        let begin = begin.unwrap();
        let mut end = end.unwrap() + 1;
        // Move the end to valid index
        while !&utf8.is_char_boundary(end) {
            end += 1;
        }
        dump(&utf8[begin..end]);
    }

    // Note that no copy is made, we use `&str`
    let words: Vec<&str> = utf8.split_whitespace().collect();
    println!("Words {:?}", words);
    let words: Vec<&str> = utf8.split(" ").collect();
    println!("Words {:?}", words);

    // Filter characters
    let stripped: String = utf8.chars()
        .filter(|ch| !ch.is_whitespace()).collect();
    println!("No whitespace {}", stripped);
}

fn dump(value: &str) {
    println!("Value '{}'", value);
}

