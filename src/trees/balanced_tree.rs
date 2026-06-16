// Note: I've changed tree data structure than the given one in NeetCode making it read-only
// as we don't need to mutate it.

pub struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Time O(n): DFS recursion
// Space O(h): where h is tree height, best case O(LogN), worst O(N)

pub fn is_balanced(root: Option<Box<TreeNode>>) -> bool {
    let mut balanced = true;
    height_dfs(&root, &mut balanced);

    balanced
}

fn height_dfs(node: &Option<Box<TreeNode>>, balanced: &mut bool) -> i32 {
    if !*balanced {
        return 0;
    }

    let Some(node) = node else {
        return 0;
    };

    let left_height = height_dfs(&node.left, balanced);
    let right_height = height_dfs(&node.right, balanced);

    if left_height.abs_diff(right_height) > 1 {
        *balanced = false;
    }

    // Height is the maximum between nodes height + the current node
    1 + left_height.max(right_height)
}
