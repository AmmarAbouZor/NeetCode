use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// We need to build a range for each value with left and right bounds
// This range will get updated on each level in the branch based on the
// parent node value.

// Time & Space: O(N) DFS Traverse

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = Vec::new();
    if let Some(node) = root {
        stack.push((i32::MIN, node, i32::MAX));
    }

    while let Some((l_bound, node, r_bound)) = stack.pop() {
        let node_ref = node.borrow();
        let node_val = node_ref.val;

        if node_val <= l_bound || node_val >= r_bound {
            return false;
        }

        if let Some(left) = node_ref.left.as_ref() {
            stack.push((l_bound, Rc::clone(left), node_val.min(r_bound)));
        }

        if let Some(right) = node_ref.right.as_ref() {
            stack.push((node_val.max(l_bound), Rc::clone(right), r_bound));
        }
    }

    true
}
