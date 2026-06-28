# Trees Notes

## DFS vs BFS

Use DFS for path/subtree properties:

- max depth
- diameter
- balanced tree
- validate BST
- good nodes
- invert tree

Use BFS for level properties:

- level order traversal
- right side view
- minimum depth

DFS space: `O(h)` where `h` is height. Balanced: `O(log n)`, skewed: `O(n)`.

BFS space: `O(w)` where `w` is max width. Worst case: `O(n)`.

## Rust tree representation

LeetCode trees often use:

```rust
Option<Rc<RefCell<TreeNode>>>
```

Use:

```rust
let node_ref = node.borrow();
let node_ref = &mut *node.borrow_mut();
```

When returning/cloning nodes, make sure temporary borrows are dropped first. A local scope can be cleaner than manual `drop`.

## Invert Binary Tree

Swap left/right for every node.

DFS stack is a good default because order does not matter and it avoids recursion depth.

BFS queue also works. Queue convention:

```rust
push_back
pop_front
```

## Max Depth

DFS tracks depth with node:

```rust
stack.push((depth + 1, child));
```

Time: `O(n)`, space: `O(h)` for DFS.

## Same Tree / Subtree

Same tree: compare structure and values recursively. Time: `O(n)`, space: `O(h)`.

Subtree: for each node in root, check equality with subroot.

Time: `O(n * m)` worst case. Space: `O(h1 + h2)` recursion stack.

## Balanced Tree

DFS returns height while updating/checking balanced state.

Height of node:

```rust
1 + max(left_height, right_height)
```

Balanced if height difference is at most 1 at every node.

Time: `O(n)`, space: `O(h)`.

## Diameter of Binary Tree

DFS returns height, not diameter.

At each node:

```rust
diameter = left_height + right_height
height = 1 + max(left_height, right_height)
```

Common pitfall: returning diameter from DFS instead of returning height.

## Binary Tree Maximum Path Sum

DFS similar to tree diameter.

At each node, compute left/right gains and ignore negative gains with `max(0)`.

Update global answer with the best path passing through this node:

```text
node.val + left_gain + right_gain
```

Return only the best one-sided path to the parent:

```text
node.val + max(left_gain, right_gain)
```

The global answer must start from a real node value, not `0`, to handle all-negative trees.

Time: `O(n)`, space: `O(h)` recursion stack.

## Good Nodes

Track maximum value seen on current path.

Node is good if:

```rust
node.val >= max_so_far
```

Time: `O(n)`, space: `O(h)` DFS or `O(n)` worst-case stack.

## Validate BST

Carry valid range for each node:

```rust
lower < node.val < upper
```

For left child, upper becomes current value. For right child, lower becomes current value.

Time: `O(n)`, space: `O(h)`.

## Kth Smallest in BST

In-order traversal gives sorted order:

```text
left -> node -> right
```

Time: `O(h + k)`, worst case `O(n)`. Space: `O(h)`.

## Construct Tree from Preorder and Inorder

Preorder gives the root first. Inorder tells how to split left and right subtrees.

```text
preorder: root, left subtree, right subtree
inorder:  left subtree, root, right subtree
```

Use a HashMap from value to inorder index to avoid searching every time.

Keep one shared `pre_idx`:

```rust
let root_val = preorder[pre_idx];
pre_idx += 1;
```

Each recursive call creates one node, so `pre_idx` only moves by one. The left recursion consumes all left-subtree preorder values before the right recursion starts.

Use half-open inorder bounds `[l, r)` to avoid `usize` underflow:

```rust
left:  [l, mid)
right: [mid + 1, r)
```

Time: `O(n)` with the map. Space: `O(n)` for the map/output and `O(h)` recursion stack.

## Serialize and Deserialize Binary Tree

Use preorder DFS with null markers.

```text
node,left,right
1,2,n,n,3,n,n
```

During deserialize, consume tokens from left to right. Each recursive call reads one token: `n` returns `None`, otherwise create the node and recursively build left and right.

Iterator version is clean because each subtree consumes exactly its own tokens.

Time: `O(n)`, space: `O(h)` recursion stack, excluding the output string.

## Lowest Common Ancestor in BST

Use BST ordering:

- if both targets are smaller, go left
- if both are larger, go right
- otherwise current node is LCA

Time: `O(h)`, space `O(1)` iterative or `O(h)` recursive.

## Level Order / Right Side View

For BFS by level, capture queue length at start of level:

```rust
let level_len = queue.len();
for _ in 0..level_len { ... }
```

Right side view: last node of each level in BFS, or DFS right-first and record first node seen at each level.
