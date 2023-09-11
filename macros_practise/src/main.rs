macro_rules! our_macro {
    () => {
        1 + 1
    };
    (something 4 u dear) => {
        println!("non sense");
    };
    ($e1: expr, $e2: expr) => {
        $e1 + $e2;
    };
}

macro_rules! input {
    ($t: ty) => {
        {
            let mut var = String::new();
            std::io::stdin()
            .read_line(&mut var)
            .expect("Failed to read input");

            let n:$t = var.trim().parse().expect("Invalid Input");
        }
    };
}
macro_rules! some_macro {
    ($var: ident) => {
        $var += 1;
    };
}

macro_rules! create_function {
    ($func_name: ident, $input: ident, $type_input: ty) => {
        fn $func_name($input: $type_input){
            println!("The function {} is called with an input {} and type {}",stringify!($func_name),stringify!($input), stringify!($type_input) );
        }
    };
}

create_function!(some, x, i32);
fn main() {
    our_macro!();
    our_macro!(something 4 u dear);

    // let input = input!(i32);
    println!("{}", our_macro!(2,2));
    let mut x = 10;
    some_macro!(x);
    println!("{}", x);

    some(20);
}
