fn main() {
    //scalar types
    let x = 3;
    println!("The value of x is: {}", x);

    let x = 3.2;
    println!("The value of x is: {}", x);

    //character type
    let x = 'x';
    println!("The value of x is: {}", x);
    let emoji = '😻';
    println!("The value of emoji is: {}", emoji);

    let heart = '❤';
    println!("The value of heart is: {}", heart);

    //compound types
    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    println!("The value of tup is: {:#?}", tup);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // println!("The value of tup.0 is: {}", tup.0);

    //array
    let a = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);
    // println!("The value of a is: {:#?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    let a = [3; 5];
    println!("The value of a is: {:?}", a);

    let a = [1, 2, 3, 4, 5];
    println!("The value of a[0] is: {}", a[0]);

    // let index = 10;
    // let element = a[index];

    // println!("The value of element is: {}", element);
}
