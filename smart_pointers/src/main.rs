//----------------------------
//  Smart Pointers
//      - Box Smart Pointer
//      -- Use case of Box Smart Pointer
//      -- Custom Defined Smart Pointer
//---------------------------

use std::ops::Deref;
// use crate::List::{Cons, Nil};

// fn main() {
//     let single_value = Box::new(0.625);
//     let x = 0.625;
//     println!(" Are these equal {}", x == *single_value);
//
//     let mut stack_var = 3;
//     let stack_ref = &stack_var;
//
//     let heap_var = Box::new(stack_var);
//     stack_var = 10;
//     println!("stack_var = {} \n heap_var = {} ", stack_var, heap_var);
// }
// #[derive(Debug)]
// enum List{
//     Cons(i32, Option<Box<List>>),
//     Nil
// }
// fn main(){
//     let list = List::Cons(10,
//                           Some(Box::new(Cons(20,
//                                              Some(Box::new(Cons(50,
//                                                                 None)))))));
//     println!("{:?}", list);
// }
#[derive(Debug)]
struct MySmartPointer{
    value: i32
}
impl MySmartPointer{
    fn new(x:i32) -> MySmartPointer{
        MySmartPointer{
            value: x
        }
    }
}
impl Deref for MySmartPointer{
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl Drop for MySmartPointer{
    fn drop(&mut self) {
        println!("Dropping the pointer for {}",self.value);
    }
}
fn main(){
    let a = 50;
    let b = &a;
    println!("{}", 50 == a);
    println!("{}", 50 == *b);
    // println!("{}", a == b);
    let ptr = MySmartPointer::new(20);
    println!("{}", 20 == *ptr);
    println!("{:?}", ptr);
    let ptr2 = MySmartPointer::new(30);
    drop(ptr);

}