fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    // Uncommenting this will result to borrow error, because
    // s.clear() requires mujtable borrow, but first_word requires
    // immutable borrow
    // s.clear();

    println!("{word} {s}");
}

fn first_word(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[..i];
        }
    }

    return &some_string[..];
}
