use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Preorder DFS serialization with null markers.
// Each node is written as `value,left,right`; missing children are written as `n`.
// Example output: `1,2,n,n,3,n,n`.
// Deserialization consumes the same token stream from left to right.
// Each recursive call reads one token, then recursively rebuilds left and right.
// Time: O(n), space: O(h) recursion stack, excluding the output string.

#[derive(Default)]
pub struct Codec;

impl Codec {
    pub fn new() -> Self {
        Self
    }

    pub fn serialize(&self, node: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut output = String::new();
        ser_dfs(node.as_ref(), &mut output);

        if output.ends_with(',') {
            output.pop();
        }

        output
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut items = data.split(',');
        des_dfs(&mut items)
    }
}

fn ser_dfs(node: Option<&Rc<RefCell<TreeNode>>>, output: &mut String) {
    use std::fmt::Write as _;

    let Some(node) = node else {
        output.push_str("n,");
        return;
    };

    let node_ref = node.borrow();
    write!(output, "{},", node_ref.val).expect("Write to string never fail.");

    ser_dfs(node_ref.left.as_ref(), output);
    ser_dfs(node_ref.right.as_ref(), output);
}

// Consumes exactly the tokens needed to rebuild one subtree.
fn des_dfs<'a>(items: &mut impl Iterator<Item = &'a str>) -> Option<Rc<RefCell<TreeNode>>> {
    let txt = items.next()?;

    if txt == "n" {
        return None;
    }

    let val: i32 = txt.parse().expect("Value either number or n");

    let mut node = TreeNode::new(val);

    node.left = des_dfs(items);

    node.right = des_dfs(items);

    Some(Rc::new(RefCell::new(node)))
}
