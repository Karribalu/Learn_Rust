fn main() {
    let str = String::from("hello world");
    let str_literal = "hello world";
    let wordIndex = first_word(&str);
    let word = first_word_string(&str);
    let temp = first_word(&str[..]);
    println!("{} is the index for first word", wordIndex);
    println!("{} is the first word", word);
    println!("{} is the first word", temp);

    string_slices();
}
fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn string_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..];

    println!("{} {}", hello, world);
}

fn first_word_string(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
