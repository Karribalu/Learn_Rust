fn main() {
    let mut s = String::from("hello"); // s comes into scope
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);
}
