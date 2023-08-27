use std::{collections::HashMap, hash::Hash};
fn main() {
    let mut persons: HashMap<&str, i32> = HashMap::new();
    persons.insert("bala", 23);
    persons.insert("shiva", 26);
    persons.insert("anoosha", 22);

    println!(
        "The age of anoosha is {:?}",
        persons.get("anoosha").unwrap()
    );
    println!(
        "The age of anoosha is {:?}",
        persons.get("anoosh").unwrap_or(&0)
    );
    let (key, value) = persons.get_key_value("anoosha").unwrap();
    println!("{} {}", key, value);
    array_count();
}

fn array_count() {
    let vector = vec![5, 5, 4, 3, 5, 2, 5, 6, 7, 8, 3, 2];
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in &vector {
        let value = map.entry(*i).or_insert(0);
        *value += 1;
    }

    println!("{:?}", map);
}
