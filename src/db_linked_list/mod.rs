mod test;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub prev: Option<Rc<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
    pub data: i32,
}

impl Node {
    pub fn new(data: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            data,
        }))
    }
}

pub fn insert(node: &Rc<RefCell<Node>>, data: i32) {
    let mut current = node.clone();
    loop {
        let next = {
            let borrowed = current.borrow();
            match borrowed.next {
                Some(ref next) => next.clone(),
                None => break,
            }
        };
        current = next;
    }

    let new_node = Node::new(data);
    new_node.borrow_mut().prev = Some(current.clone());
    current.borrow_mut().next = Some(new_node);
}
pub fn print_list(head: &Rc<RefCell<Node>>) {
    let mut current = Some(head.clone());
    while let Some(node) = current {
        print!("{} ", node.borrow().data);
        current = node.borrow().next.clone();
    }
    println!();
}
