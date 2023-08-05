#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let home = IpAddrKind::V4(String::from("121.1.1.0"));
    let some = IpAddrKind::V6(String::from("some"));
    println!("{:#?} \n{:#?}", home, some);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    assert_eq!(y.is_some(), true);

    //Match with Enums
    let coin = Coin::Nickel;
    println!("coin is {}", value_in_cents(coin));
    println!(
        "coin is {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    let op = match x {
        2 => "one",
        6 => "five",
        _ => "other",
    };
    println!("op is {}", op);

    // Match with Options
    let five = Some(5);
    let six = match_with_options(five);
    let none = match_with_options(None);

    print!("{:?} {:?}", six, none);
    
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(varient) => {
            println!("found one varient {:?}!", varient);
            25
        }
    }
}

fn match_with_options(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
