use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// The key here to consider an in-order DFS traversal (left-middle-right) while
// Counting nodes, to make use for the binary search tree properties.

// Time: O(h + k), where h is the tree height. We may first descend h nodes
// to reach the leftmost value, then visit up to k nodes in sorted order.
// Worst case is O(n), for example when k == n or the tree is skewed.
//
// Space: O(h) from the recursion stack. Worst case is O(n) for a skewed tree,
// and O(log n) for a balanced tree.

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut visited = 0;
    dfs(&root, k as usize, &mut visited).unwrap()
}

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, k: usize, visited_count: &mut usize) -> Option<i32> {
    let node = root.as_ref()?;
    let node_ref = node.borrow();

    if let Some(res) = dfs(&node_ref.left, k, visited_count) {
        return Some(res);
    }

    *visited_count += 1;

    if *visited_count == k {
        return Some(node_ref.val);
    }

    dfs(&node_ref.right, k, visited_count)
}
