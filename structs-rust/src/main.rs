struct User {
    name: String,
    age: u32,
    active: bool,
}
struct User2 {
    name: String,
    age: u32,
    active: bool,
}
fn main() {
    let user1 = User {
        active: true,
        name: String::from("bala subramanyam"),
        age: 30,
    };

    println!("{}", user1.name);
}
