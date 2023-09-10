use std::fmt;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug)]
struct MySmartPointer<T: fmt::Debug >{
    value: T
}
impl<T: fmt::Debug> MySmartPointer<T>{
    fn new(x:T) -> MySmartPointer<T>{
        MySmartPointer{
            value: x
        }
    }
}
impl<T: fmt::Debug> Deref for MySmartPointer<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T: fmt::Debug> Drop for MySmartPointer<T>{
    fn drop(&mut self) {
        println!("Dropping the pointer for {}",self.value);
    }
}
fn my_fn(str: &str){
    println!("String is {}", str);
}
fn main() {
    //Corecion
    let sptr_1 = MySmartPointer::new("Bala");
    my_fn(&sptr_1);
    let some_vector = MySmartPointer::new(vec![10,20,30]);
    for number in &*some_vector{
        println!("{}", number);
    }
}
