struct Stack<T> {
    items: Vec<T>,
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
}
fn main() {
    let mut stack = Stack::new();
    stack.push(100);
    stack.push(200);
    println!("Stack size: {}", stack.size());
    while let Some(item) = stack.pop() {
        println!("Popped: {}", item);
    }
    println!("Is stack empty: {}", stack.is_empty());

}
