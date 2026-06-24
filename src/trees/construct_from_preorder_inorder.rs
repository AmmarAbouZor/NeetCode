use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        let left = None;
        let right = None;
        Self { val, left, right }
    }
}

// Optimized solution using HashMap to avoid having a linear lookup on each node.
// However this solution is more complex to understand. Therefore, it's better to start with
// the solution below and continue with this one after that

// Time: O(n), each node is processed once and each HashMap lookup is O(1) average.
// Space: O(n), for the HashMap and output tree. Recursion stack is O(h), worst-case O(n).

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let indices_map: HashMap<i32, usize> = inorder
        .iter()
        .enumerate()
        .map(|(idx, num)| (*num, idx))
        .collect();
    let mut pre_idx = 0;

    dfs(&preorder, &indices_map, &mut pre_idx, 0, preorder.len())
}

fn dfs(
    preorder: &[i32],
    indices: &HashMap<i32, usize>,
    pre_idx: &mut usize,
    l: usize,
    r: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    if l >= r {
        return None;
    }

    let mid_val = preorder[*pre_idx];
    let mid = *indices
        .get(&mid_val)
        .expect("Constraints guarantee the value exists");
    *pre_idx += 1;
    let mut node = TreeNode::new(mid_val);

    node.left = dfs(preorder, indices, pre_idx, l, mid);
    node.right = dfs(preorder, indices, pre_idx, mid + 1, r);

    Some(Rc::new(RefCell::new(node)))
}

// Simpler O(n^2) version. Useful for understanding the recursive split before
// adding the HashMap optimization.

// The first preorder value is the root. Find that value in inorder to split
// the inorder slice into left and right subtrees. The size of the left inorder
// slice tells us how much of preorder belongs to the left subtree.

// Time: O(n^2): We iterate through each item and we search for mid position for each node.
// Space: O(n): For output

pub fn build_tree_no_map(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_tree_recr(&preorder, &inorder)
}

fn build_tree_recr(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    let mut root = TreeNode::new(preorder[0]);
    let mid = inorder
        .iter()
        .position(|n| *n == root.val)
        .expect("Constraints guarantee the value exists");

    root.left = build_tree_recr(&preorder[1..mid + 1], &inorder[..mid]);
    root.right = build_tree_recr(&preorder[mid + 1..], &inorder[mid + 1..]);

    Some(Rc::new(RefCell::new(root)))
}
