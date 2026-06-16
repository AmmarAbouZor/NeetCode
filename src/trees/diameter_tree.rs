use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// Time: O(N) DFS with recursion
// Space: O(h) where h is tree height. balanced O(LogN). Worst case O(N)

// Common pitfall is to return the best possible height from  the dfs function and not the diameter.

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_diameter = 0;
    diameter_dfs(&root, &mut max_diameter);

    max_diameter
}

/// This function return the best possible height from this node, which means either left or right
/// but not the diameter in this node.
fn diameter_dfs(root: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
    let Some(node) = root else {
        return 0;
    };

    let node_ref = node.borrow();

    let left_sum = diameter_dfs(&node_ref.left, max_diameter);
    let right_sum = diameter_dfs(&node_ref.right, max_diameter);

    // Both left and right sides are included in max_diameter as
    // It can be between any two nodes in tree
    *max_diameter = (*max_diameter).max(left_sum + right_sum);

    // The function should return either left side or right side but
    // not the both of them
    1 + left_sum.max(right_sum)
}
