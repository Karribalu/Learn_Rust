struct User {
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

    let user2 = User {
        name: String::from("Anoosha"),
        ..user1
    };

    println!("{} {} {}", user1.active, user1.age, user1.name);
}
