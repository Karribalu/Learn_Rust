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
    let longest_word: &str;
    {
        let str2 = String::from("world");
        longest_word = longest(&str1, &str2);
        println!("{}", longest_word);
    }

    fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }
}

pub fn lifetime_4() {
    use std::fmt::Display;
    fn logest_with_ann<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    logest_with_ann("some", "someother", String::from("hello"));
}
