use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// Time O(n * m): where n is node count in root and m is node count in sub_root.
// Space: O(h1 + h2), where h1 and h2 are the heights of root and sub_root.
// Worst case: O(n + m) for skewed trees.
// Balanced trees: O(log n + log m).

// Traverse the main tree and check on each node if it's equal to the sub_root
pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if is_equal(&root, &sub_root) {
        return true;
    }

    let Some(node) = root else {
        return false;
    };

    let node_ref = node.borrow();

    is_subtree(node_ref.left.clone(), sub_root.clone())
        || is_subtree(node_ref.right.clone(), sub_root.clone())
}

fn is_equal(t1: &Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (t1, t2) {
        (None, None) => true,
        (Some(t1), Some(t2)) => {
            let node1 = t1.borrow();
            let node2 = t2.borrow();

            node1.val == node2.val
                && is_equal(&node1.left, &node2.left)
                && is_equal(&node1.right, &node2.right)
        }
        _ => false,
    }
}
