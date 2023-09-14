use std::num::ParseIntError;

fn parse_string(input: &str) -> Result<i32, ParseIntError>{
    let integer = input.parse::<i32>()?;
    println!("the value is {:?} and the int is {:?}", input, integer);

    Ok(integer)
}
fn main() {
    println!("{:?} {:?}", parse_string("some"),parse_string("33"));
}
