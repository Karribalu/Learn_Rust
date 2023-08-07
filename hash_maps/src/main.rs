use std::hash::Hash;

fn main() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    println!("{blue_score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{field_name}");
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    println!("{}", scores.entry(String::from("Blue")).or_insert(100));
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
