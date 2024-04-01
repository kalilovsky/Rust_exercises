use std::fmt::Debug;

#[derive(Debug)]
pub struct CustomLinkList<T: Debug  > {
    head: Option<Box<Node<T>>>
}

impl <T: Debug  > CustomLinkList<T> {
    pub fn new() -> Self {
        CustomLinkList {
            head: None
        }
    }

    pub fn insert(&mut self, value: T){
        let old_head = self.head.take();
        let new_head = Some(Box::new(Node::new(value, old_head)));
        self.head = new_head
    }

    pub fn remove(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
            None => None
        }
    }

    pub fn custom_print(&self) {
        let mut next = &self.head;

        loop {
            match next {
                Some(node) => {
                    println!("The value is {:?}", node.value);
                    next = &node.next;
                }
                None => {
                    println!("The limit has been reached");
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node<T: Debug  > {
    value: T,
    next: Option<Box<Node<T>>>
}

impl <T: Debug> Node<T> {
    fn new(value : T, next: Option<Box<Node<T>>>) -> Self{
        Node{ value, next}
    }
}