use std::fmt::Debug;

#[derive(Debug)]
struct LinkedList<T: Debug + Copy>{
    head: pointer<T>
}
#[derive(Debug)]
struct Node<T: Debug + Copy>{
    element: T,
    next: pointer<T>
}

type pointer<T> = Option<Box<Node<T>>>;

impl<T: Debug + Copy> LinkedList<T>{
    fn create_empty_list() -> LinkedList<T>{
        LinkedList{
            head: None
        }
    }
    fn add(&mut self, element: T){
        let previous_head = self.head.take();
        let new_head = Box::new(Node{
            element: element,
            next: previous_head
        });
        self.head = Some(new_head);
    }
    fn remove(&mut self) -> Option<T>{
        let previous_head = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None
        }
    }

    fn peek(&self) -> Option<T>{
        match &self.head{
            Some(head_node) => Some(head_node.element),
            None => None
        }
    }

    fn printing(&self){
        let mut list_traversal = &self.head;
        loop {
            match list_traversal {
                Some(Node) => {
                    println!("{:?}", Node.element);
                    list_traversal = &Node.next;
                }
                None => break
            }
        }
    }
}
fn main() {
    // let linked_list = LinkedList{head: Node{
    //     element: 20,
    //     next: Some(Box::new(Node{
    //         element: 30,
    //         next: None
    //     }))
    // }};

    // let empty_head = LinkedList{head: None};
    // let linked_list = LinkedList{head: Some(Box::new(Node{
    //     element: 30,
    //     next: Some(Box::new(Node{
    //         element:40,
    //         next: None
    //     }))
    // }))};
    // first element
    // println!("{:?}", &linked_list.head.unwrap().element);

    // second_element
    // println!("{:?}", linked_list.head.unwrap().next.unwrap().element);

    let mut linked_list = LinkedList::create_empty_list();
    linked_list.add(10);
    linked_list.add(30);
    linked_list.add(5);
    linked_list.add(60);

    println!("{:?}", linked_list);

    linked_list.remove();
    println!("{:?}", linked_list);

    println!("peek {:?}", linked_list.peek());

    linked_list.printing();


    let mut float_list = LinkedList::create_empty_list();
    float_list.add(10.5);
    float_list.add(20.5);
    float_list.add(45.8);
    println!("{:?}", float_list);

}
