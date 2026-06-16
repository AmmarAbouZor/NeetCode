use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// O(N): We traverse over all nodes with breadth first.
// O(H): where H is tree height. For balanced: logN. Worst case is N for skewed trees.
pub fn invert_tree_dfs_recursion(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root?;
    let mut node_ref = node.borrow_mut();

    let left = invert_tree_dfs_recursion(node_ref.left.take());
    let right = invert_tree_dfs_recursion(node_ref.right.take());

    node_ref.left = right;
    node_ref.right = left;

    drop(node_ref);

    Some(node)
}

// Time: O(N) dfs traverse
// O(H): where H is tree height. For balanced: logN. Worst case is N for skewed trees.
pub fn invert_tree_dfs_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root?;
    let mut stack = vec![node.clone()];

    while let Some(node) = stack.pop() {
        // borrow_mut returns RefMut<T> which can't be converted automatically to &mut
        // Therefore, we need to deref it as &mut
        let node_ref = &mut *node.borrow_mut();

        std::mem::swap(&mut node_ref.left, &mut node_ref.right);

        if let Some(left) = node_ref.left.as_ref() {
            stack.push(left.clone());
        }
        if let Some(right) = node_ref.right.as_ref() {
            stack.push(right.clone());
        }
    }

    Some(node)
}

// Time: O(N) bfs traverse
// O(W): where W is tree  maximum width. Worst case is N.
pub fn invert_tree_bfs_queue(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root?;
    let mut queue = std::collections::VecDeque::with_capacity(1);
    queue.push_back(node.clone());

    while let Some(node) = queue.pop_front() {
        let node_ref = &mut *node.borrow_mut();
        // borrow_mut returns RefMut<T> which can't be converted automatically to &mut
        // Therefore, we need to deref it as &mut
        std::mem::swap(&mut node_ref.left, &mut node_ref.right);

        if let Some(left) = node_ref.left.as_ref() {
            queue.push_back(left.clone());
        }
        if let Some(right) = node_ref.right.as_ref() {
            queue.push_back(right.clone());
        }
    }

    Some(node)
}
