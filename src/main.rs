use verus_algorithm::*;

fn main() {
    let head = Node::new(1);

    push_back(&head, 2);
    push_back(&head, 3);
    push_back(&head, 4);

    println!("Double Linked List contents:");
    print_list(&head);

    // 리스트의 구조 확인
    let second = head.borrow().next.as_ref().unwrap().clone();
    let third = second.borrow().next.as_ref().unwrap().clone();
    let fourth = third.borrow().next.as_ref().unwrap().clone();

    println!("Checking node connections:");
    println!(
        "Second node: prev = {}, current = {}, next = {}",
        second.borrow().prev.as_ref().unwrap().borrow().data,
        second.borrow().data,
        second.borrow().next.as_ref().unwrap().borrow().data
    );
    println!(
        "Third node: prev = {}, current = {}, next = {}",
        third.borrow().prev.as_ref().unwrap().borrow().data,
        third.borrow().data,
        third.borrow().next.as_ref().unwrap().borrow().data
    );
    println!(
        "Fourth node: prev = {}, current = {}, next = None",
        fourth.borrow().prev.as_ref().unwrap().borrow().data,
        fourth.borrow().data
    );
}
