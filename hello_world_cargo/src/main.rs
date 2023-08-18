fn main() {
    println!("Hello, world!");
    println!(
        "max in i32 {} max in i64 {} in i128 {}",
        i32::MAX,
        i64::MAX,
        i128::MAX
    );

    let val: i128 = 9223372036854775807 * 9223372036854775807;
    println!("{}", val);
}
