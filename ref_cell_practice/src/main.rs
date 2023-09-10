use std::cell::RefCell;
use std::rc::Rc;

//--------------------------------------
//              RefCell
//--------------------------------------
fn main() {
    // let mut x = 10;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;
    // println!("{}{}", x1,x2);

    // let mut a = RefCell::new(20);
    // let b = a.borrow();
    // let c = a.borrow();
    // let d = a.borrow_mut();
    let var = Rc::new(RefCell::new(String::from("bala")));
    let mut_var = Rc::clone(&var);

    *mut_var.borrow_mut() = String::from("Anusha");
    println!("var content {:?}", var);
    *var.borrow_mut() = String::from("Some");
    println!("mut var content {:?}", mut_var);

}
