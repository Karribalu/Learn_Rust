#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 30,
    };
    println!("the area is {}", area(&rect1));
    print!("{:#?}", rect1);
    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
