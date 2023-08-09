fn main() {
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1 + &s2;
    println!("{s3}");
    // println!("{s1}");
    println!("{s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}
