use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

// BFS will fill breadths in the queue one after another, using this
// We can avoid having to save the depth with the value and we just iterate
// Over all items at the start of each level and push the level to the results.

// Time and Space: O(N) as we traverse over all items and save them all in results vector

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    let mut queue = std::collections::VecDeque::new();
    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        // Get queue len at the start ensuring we will iterate
        // through items of this level only.
        let queue_len = queue.len();
        let mut level = Vec::with_capacity(queue_len);

        for _ in 0..queue_len {
            let node = queue.pop_front().expect("Queue length is checked");
            let node_ref = node.borrow();

            level.push(node_ref.val);

            if let Some(left) = node_ref.left.as_ref() {
                queue.push_back(Rc::clone(left));
            }

            if let Some(right) = node_ref.right.as_ref() {
                queue.push_back(Rc::clone(right));
            }
        }

        res.push(level);
    }

    res
}

// This is working but we have to save the index with the value
// Making it similar to dfs approach.
pub fn level_order_idx(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();

    let Some(node) = root else {
        return res;
    };

    let mut queue = std::collections::VecDeque::new();

    queue.push_back((0, node));

    while let Some((idx, node)) = queue.pop_front() {
        let node_ref = node.borrow();
        while idx >= res.len() {
            res.push(Vec::new());
        }
        res[idx].push(node_ref.val);

        if let Some(left) = node_ref.left.as_ref() {
            queue.push_back((idx + 1, Rc::clone(left)));
        }

        if let Some(right) = node_ref.right.as_ref() {
            queue.push_back((idx + 1, Rc::clone(right)));
        }
    }

    res
}
