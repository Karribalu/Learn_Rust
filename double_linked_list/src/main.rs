use std::cell::RefCell;
use std::rc::{Rc,Weak};
#[derive(Debug)]
struct Node<T>{
    element: T,
    next: Pointer<T>,
    prev: Pointer<T>
}
#[derive(Debug)]
struct List<T>{
    head: Pointer<T>,
    tail: Pointer<T>
}
type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: std::fmt::Display> Node<T>{
    fn new(element: T) -> Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(Node{
            element,
            prev:None,
            next:None
        }))
    }
}

impl<T: std::fmt::Display> List<T>{
    fn new() -> Self{
        List{
            head: None,
            tail: None
        }
    }

    fn push_front(&mut self, element: T){
        let new_node = Node::new(element);
        match self.head.take() {
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            },
            Some(old_head) =>{
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_node);
            }
        }
    }

    fn push_back(&mut self, element: T){
        let new_tail = Node::new(element);
        match self.tail.take() {
            None =>{
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }

            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
}
fn main() {
    let mut dlist = List::new();
    dlist.push_front(10);
    dlist.push_front(20);
    // dlist.push_back(30);

}
