fn main() {
    let some_option = Some(String::from("bala"));
    let some_1 = &some_option;
    let some_2 = some_option.as_ref();

    try_me(some_2);
    println!("{:?}", some_2);
}
fn try_me(option_name: Option<&String>){
    match option_name {
        Some(inner_value) => println!("option value is {}", inner_value),
        None => println!("no name")
    }
}
