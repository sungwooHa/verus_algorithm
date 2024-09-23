use vstd::prelude::*;
use vstd::ptr::PPtr;

verus! {

    struct Node {
        prev: Option<PPtr<Node>>,
        next: Option<PPtr<Node>>,
        data: i32,
    }

    impl Node {
        pub open spec fn well_formed(&self) -> bool {
            //prev, next 패턴에 따른 스펙 정의
            match (&self.prev, &self.next) {
                (Some(prev), Some(next)) =>
                    prev.next.is_some() && next.prev.is_some() &&
                        prev.next.unwrap() == self && next.prev.unwrap() == self,
                (None, Some(next)) => next.prev.is_some() && next.prev.unwrap() == self,
                (Some(prev), None) => prev.next.is_some() && prev.next.unwrap() == self,
                (None, None) => true,
            }
        }

        pub open spec fn is_tail(&self) -> bool {
            self.next.is_none()
        }

        pub open spec fn is_head(&self) -> bool {
            self.prev.is_none()
        }
    }

    pub exec fn push_back(node: &PPtr<Node>, data: i32) -> (result: PPtr<Node>)
        requires
            node.well_formed(),
        ensures
            node.well_formed(),
            result.well_formed(),
            result.is_tail(),
            result.prev.is_some(),
            result.data == data
    {

        let mut current = node;
        while let Some(next) = current.next {
            invariant(current.well_formed());
            current = next;
        }

        let new_node = PPtr::new(Node{
            prev: Some(current),
            next: None,
            data
        });
        current.next = Some(new_node);

        assert(node.well_formed());
        assert(new_node.well_formed());
        new_node
    }

    pub exec fn new_node(data: i32) -> (result: PPtr<Node>)
        ensures
            result.well_formed(),
            result.is_tail(),
            result.is_head(),
            result.data == data
    {
        PPtr::new(Node {
            prev: None,
            next: None,
            data
        })
    }
}
