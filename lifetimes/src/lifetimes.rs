pub fn lifetime_1() {
    let r = 10;
    {
        let x = 10;
        // r = &x;
    }
    println!("r: {}", r);
}
pub fn lifetime_2() {
    let x = 10;
    let r = &x;
    println!("r: {}", r);
}
pub fn lifetime_3() {
    let str1 = String::from("hello");
    let str2 = String::from("world");
    let longest = longest(&str1, &str2);
    println!("{}", longest);

    fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }
}


