use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// Time: O(N) DFS traverse
// Height: O(h) where h is the height of the smaller tree

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let (node1, node2) = match (p, q) {
        (Some(p1), Some(q2)) => (p1, q2),
        (None, None) => return true,
        _ => return false,
    };

    let n1 = node1.borrow();
    let n2 = node2.borrow();

    n1.val == n2.val
        && is_same_tree(n1.left.clone(), n2.left.clone())
        && is_same_tree(n1.right.clone(), n2.right.clone())
}
