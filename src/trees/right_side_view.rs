use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// With BFS we can iterate on each level at a time. Items that are
// seen from the right are actually the last item of each level.

// Time O(N): BFS traversal
// Space O(w + h): w tree largest width for BFS , h for results
// Space O(N) on skew trees.

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();

    let mut queue = std::collections::VecDeque::new();

    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        let q_len = queue.len();
        for idx in 0..q_len {
            let node = queue.pop_front().expect("Queue bounds are checked");
            let node_ref = node.borrow();

            if idx == q_len - 1 {
                res.push(node_ref.val);
            }

            if let Some(left) = node_ref.left.as_ref() {
                queue.push_back(Rc::clone(left));
            }

            if let Some(right) = node_ref.right.as_ref() {
                queue.push_back(Rc::clone(right));
            }
        }
    }

    res
}

// It's also possible to solve with dfs by visiting the right node first always and
// provide the level. Then the first child which will hit new level will be the first
// node to the right.

// Time O(N)
// Space O(N) on skew trees.
pub fn right_side_view_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    dfs(&root, 0, &mut res);

    res
}

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, res: &mut Vec<i32>) {
    let Some(node) = root.as_ref() else {
        return;
    };
    let node_ref = node.borrow();

    if res.len() == level {
        res.push(node_ref.val);
    }

    dfs(&node_ref.right, level + 1, res);
    dfs(&node_ref.left, level + 1, res);
}
