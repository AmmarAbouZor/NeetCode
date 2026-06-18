// The graph can contain cycles, so we keep a map from original node value
// to cloned node. If a node was already cloned, return it instead of
// recursing again.

// Time: O(V + E), where V is nodes and E is neighbor links.
// Space: O(V), excluding the cloned graph, for the map and recursion stack.
// Output space: O(V + E).

use std::{cell::RefCell, rc::Rc};

pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            neighbors: Vec::new(),
        }
    }
}

use std::collections::HashMap;

pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    let node = node?;

    let mut cloned_map: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();

    let graph = dfs(node, &mut cloned_map);

    Some(graph)
}

fn dfs(
    node: Rc<RefCell<Node>>,
    cloned_map: &mut HashMap<i32, Rc<RefCell<Node>>>,
) -> Rc<RefCell<Node>> {
    let val = node.borrow().val;
    if let Some(node) = cloned_map.get(&val) {
        return Rc::clone(node);
    }

    let new_node = Rc::new(RefCell::new(Node::new(val)));

    // Insert node in map before any new calls for dfs function.
    cloned_map.insert(val, Rc::clone(&new_node));

    for neighbor in &node.borrow().neighbors {
        let new_neighbor = dfs(Rc::clone(neighbor), cloned_map);
        new_node.borrow_mut().neighbors.push(new_neighbor);
    }

    new_node
}
