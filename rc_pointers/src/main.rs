/*

enum List{
    Cons(i32, Rc<List>),
    Nil
}

use std::rc::Rc;
use crate::List::{Cons, Nil};
fn main() {
    // let a = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(5, Box::new(a));
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("Count after creating a is {}", Rc::strong_count(&a));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Count after creating b is {}", Rc::strong_count(&a));

    let c = Rc::new(Cons(10, Rc::clone(&a)));
    println!("Count after creating c is {}", Rc::strong_count(&a));



}
*/
use std::rc::Rc;
fn make_rc() -> Rc<String>{
    let s1 = Rc::new(String::from("hello"));
    println!("The count when RC pointer is created is {}", Rc::strong_count(&s1));
    let s2 = s1.clone();
    println!("The count when RC pointer is clone is {}", Rc::strong_count(&s1));
    s2
}

fn main(){
    let rc_string = make_rc();
    println!("The count after the string gets created is  {}", Rc::strong_count(&rc_string));
}
