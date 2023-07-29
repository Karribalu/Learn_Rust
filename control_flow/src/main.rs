fn main() {
    //if
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //if-let
    let condition = true;
    let _number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", _number);
    //if-let-different data types
    // let condition = true;
    // let _number = if condition { 5 } else { "six" };
    // println!("The value of number is: {}", _number);

    loops();
    for_loops();
}

fn loops() {
    //loop
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;
    let _result = loop {
        counter += 1;
        if counter == 10 {
            //break counter * 2;
            break counter * 2;
        }
    };
    println!("The result is {}", _result);

    //loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn for_loops() {
    //for
    let a = [10, 20, 30, 40, 50];
    println!("for loops");
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //for-range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
