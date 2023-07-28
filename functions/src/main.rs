fn main() {
    //statements and expressions
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // expression without semicolon
    };
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    //functions
    let x = ten();
    println!("The value of x is: {}", x);
}
fn ten() -> i32 {
    10
}
