use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// DFS vs BFS:
// I’ll use DFS because max depth is a path-depth property. It visits each node once and uses O(h) stack space.
// BFS would also work by counting levels, but it may hold an entire level in memory.

// Time: O(N), DFS traverse
// Space: O(h) where h is tree height. Balanced O(LogN), Worst-case O(N) like in skewed tree
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let Some(node) = root else { return 0 };
    let mut stack = vec![(1, node)];

    let mut max_d = 1;

    while let Some((depth, node)) = stack.pop() {
        max_d = max_d.max(depth);

        let node_ref = node.borrow();
        if let Some(left) = node_ref.left.as_ref() {
            stack.push((depth + 1, Rc::clone(left)));
        }
        if let Some(right) = node_ref.right.as_ref() {
            stack.push((depth + 1, Rc::clone(right)));
        }
    }

    max_d
}
