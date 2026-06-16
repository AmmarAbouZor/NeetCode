use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// Lowest common ancestor is the first node where p and q aren't on
// the same branch anymore.
// This is equivalent to the first node where one of target is smaller and the
// other is greater.

// Time & Space: O(h) where h is tree height

// Solution using iterator
pub fn lowest_common_ancestor_iter(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.as_ref()?.borrow().val;
    let q_val = q.as_ref()?.borrow().val;

    let mut cur = root;

    while let Some(node) = cur {
        let node_val = node.borrow().val;

        if p_val > node_val && q_val > node_val {
            cur = node.borrow().right.clone();
        } else if p_val < node_val && q_val < node_val {
            cur = node.borrow().left.clone();
        } else {
            return Some(node);
        }
    }

    None
}

// Solution using recursion.
pub fn lowest_common_ancestor_recr(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root = root?;
    let p_val = p.as_ref()?.borrow().val;
    let q_val = q.as_ref()?.borrow().val;

    // We need to call root.borrow() multiple time because it must be dropped
    // before returning Some(root).
    let root_val = root.borrow().val;

    if p_val > root_val && q_val > root_val {
        lowest_common_ancestor_recr(root.borrow().right.clone(), p, q)
    } else if p_val < root_val && q_val < root_val {
        lowest_common_ancestor_recr(root.borrow().left.clone(), p, q)
    } else {
        Some(root)
    }
}
