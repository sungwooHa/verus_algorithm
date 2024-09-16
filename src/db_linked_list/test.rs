#[cfg(test)]
mod tests {
    use crate::*;
    use std::rc::Rc;

    #[test]
    fn test_new_node() {
        let node = Node::new(5);
        assert_eq!(node.borrow().data, 5);
        assert!(node.borrow().prev.is_none());
        assert!(node.borrow().next.is_none());
    }

    #[test]
    fn test_insert() {
        let head = Node::new(1);
        push_back(&head, 2);
        push_back(&head, 3);

        let second = head.borrow().next.as_ref().unwrap().clone();
        let third = second.borrow().next.as_ref().unwrap().clone();

        assert_eq!(head.borrow().data, 1);
        assert_eq!(second.borrow().data, 2);
        assert_eq!(third.borrow().data, 3);

        assert!(head.borrow().prev.is_none());
        assert!(third.borrow().next.is_none());

        assert!(Rc::ptr_eq(&head, &second.borrow().prev.as_ref().unwrap()));
        assert!(Rc::ptr_eq(&second, &third.borrow().prev.as_ref().unwrap()));
    }

    #[test]
    fn test_multiple_inserts() {
        let head = Node::new(1);
        for i in 2..6 {
            push_back(&head, i);
        }

        let mut current = Some(head.clone());
        for i in 1..6 {
            let node = current.unwrap();
            assert_eq!(node.borrow().data, i);
            current = node.borrow().next.clone();
        }
        assert!(current.is_none());
    }
}
