// Linked list implementation
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }
    
    fn prepend(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        
        self.head = Some(new_node);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}

fn main() {
    let mut list = LinkedList::new();
    
    list.prepend(1);
    list.prepend(2);
    list.prepend(3);
    
    while let Some(data) = list.pop() {
        println!("Popped: {}", data);
    }
}