use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// We keep track on the max value for each branch and compare the
// current value with it.

// Time & Space: O(N) DFS traversal.

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut good_count = 0;

    let mut stack = Vec::new();
    if let Some(node) = root {
        stack.push((i32::MIN, node));
    }

    while let Some((mut max_parent, node)) = stack.pop() {
        let node_ref = node.borrow();
        if node_ref.val >= max_parent {
            good_count += 1;
            max_parent = node_ref.val;
        }

        if let Some(left) = node_ref.left.as_ref() {
            stack.push((max_parent, Rc::clone(left)));
        }

        if let Some(right) = node_ref.right.as_ref() {
            stack.push((max_parent, Rc::clone(right)));
        }
    }

    good_count
}
