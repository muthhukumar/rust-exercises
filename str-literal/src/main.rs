fn main() {
    let s = String::from("hellow world");

    // passing a slice of s, full string
    let word = first_word(&s[..]);
    println!("{}", word);

    let string_literal = "Hello world";

    let word = first_word(&string_literal[..]);
    println!("{}", word);

    let word = first_word(string_literal);
    println!("{}", word);

    let word = first_word(&string_literal[..]);
    println!("{}", word);
}

// Now here we are getting both string literal and String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
