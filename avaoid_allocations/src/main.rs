use std::mem;
#[derive(Debug)]
enum Customer{
    new{ name: String},
    loyal{name: String},
    rich{name: String}
}
fn promote(user: &mut Customer){
    use Customer::*;

    *user = match user {
        Customer::new {name} => Customer::loyal {name: mem::take(name)},
        Customer::loyal {name} => Customer::rich {name: mem::take(name)},
        Customer::rich {name} => return
    }
}
fn main() {
    let mut customer = Customer::new {name: "bala".to_string()};

    promote(&mut customer);

    promote(&mut customer);

    println!("{:?}", customer);
}
