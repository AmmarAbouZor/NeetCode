use std::{cell::RefCell, rc::Rc};

// DFS similar to tree diameter.
//
// For each node:
// - update the global answer with the best path that passes through this node:
//   node.val + left_gain + right_gain
// - return the best one-sided path that can be extended by the parent:
//   node.val + max(left_gain, right_gain)
//
// Negative child gains are ignored with max(0), because they would only reduce
// the path sum.
//
// Time: O(n), each node is visited once.
// Space: O(h) recursion stack, worst case O(n) for a skewed tree.

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(mut res) = root.as_ref().map(|n| n.borrow().val) else {
        return -1;
    };
    dfs(root.as_ref(), &mut res);
    res
}

fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
    let Some(node) = node else {
        return 0;
    };

    let node_ref = node.borrow();
    let left = dfs(node_ref.left.as_ref(), res).max(0);
    let right = dfs(node_ref.right.as_ref(), res).max(0);

    *res = (*res).max(node_ref.val + left + right);

    node_ref.val + left.max(right)
}
