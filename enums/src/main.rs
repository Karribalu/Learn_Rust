#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
fn main() {
    let home = IpAddrKind::V4(String::from("121.1.1.0"));
    let some = IpAddrKind::V6(String::from("some"));
    println!("{:#?} \n{:#?}", home, some);
}
