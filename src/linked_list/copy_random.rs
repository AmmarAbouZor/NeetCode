use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        let next = None;
        let random = None;
        Self { val, next, random }
    }
}

// The solution is simpler than the Rust code looks. The main issue is
// working with Rc<RefCell<Node>> without relying on node values being unique.

// We clone all nodes and store them in a HashMap keyed by the original node's pointer.
// Values are not guaranteed unique, so the map must use node identity, not node value.
// After that we iterate through the original list, use the nodes pointer to get their matching
// copies from the map and filling their next and random node from the map.

// Then we use the same technique to return the head by using the original pointer and the map.

// Time: O(n) we iterate the original twice:
// * First for copying/filling the HashMap
// * Second to assign next and random values for the copied values.

// Space: O(n): For the HashMap and cloned list

pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let mut map = HashMap::new();

    let mut cur = head.clone();

    // Copy all nodes filling the HashMap without assigning next/random
    while let Some(node) = cur {
        let val = node.borrow().val;
        let copied = Rc::new(RefCell::new(Node::new(val)));

        map.insert(Rc::as_ptr(&node), copied);

        cur = node.borrow().next.clone();
    }

    // Second loop to assign next/random
    let mut cur = head.clone();

    while let Some(node) = cur {
        let copied = map.get(&Rc::as_ptr(&node)).expect("Copied node must exist");
        let (next, random) = {
            let node_ref = node.borrow();
            (node_ref.next.clone(), node_ref.random.clone())
        };

        copied.borrow_mut().next = next
            .as_ref()
            .map(|next_node| Rc::clone(map.get(&Rc::as_ptr(next_node)).unwrap()));

        copied.borrow_mut().random = random
            .as_ref()
            .map(|random_node| Rc::clone(map.get(&Rc::as_ptr(random_node)).unwrap()));

        cur = next;
    }

    // Return cloned head using the pointer of the original head.
    head.as_ref()
        .map(|node| Rc::clone(map.get(&Rc::as_ptr(node)).unwrap()))
}
