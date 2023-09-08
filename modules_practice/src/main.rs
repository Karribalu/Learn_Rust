mod file_1;
mod file_2;

fn main() {
    let length = 10;
    let width = 20;
    let area = file_1::area(&length, &width);
    println!("area is {}", area);
}
