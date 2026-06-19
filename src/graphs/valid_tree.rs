// Valid tree means:
// * No cycles in graph
// * All nodes are connected

// We build adjacency list for unordered graph, which means that we will add nodes
// in both directions, then we apply dfs and any node considering that node child shouldn't
// call its parent back. We keep track on visited nodes and return false if any node is
// inserted more than once.
// Also the number of visited nodes must match the total number of nodes to ensure all nodes
// are connected.

// Time: O(V + E), where V is n and E is edges.len().
// Space: O(V + E), for the adjacency list, visited set, and recursion stack.

use std::collections::HashSet;

pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    // For a tree with n nodes, edges must be exactly n - 1
    if edges.len() != n - 1 {
        return false;
    }

    let mut adj_map = vec![Vec::new(); n];
    for pair in edges {
        let (n1, n2) = (pair[0] as usize, pair[1] as usize);
        adj_map[n1].push(n2);
        adj_map[n2].push(n1);
    }

    // It's possible to use vec![false; n] which will be more efficient
    // I use this here to demonstrate using sets as all other examples
    // are using array
    let mut visited = HashSet::new();

    resolve_dfs(0, None, &adj_map, &mut visited) && visited.len() == n
}

fn resolve_dfs(
    node: usize,
    parent: Option<usize>,
    adj_map: &[Vec<usize>],
    visited: &mut HashSet<usize>,
) -> bool {
    // Cycle detection
    if !visited.insert(node) {
        return false;
    }

    for &neighbor in &adj_map[node] {
        // Skip false cycle detection between node and its parent due to
        // unordered graph relations
        if parent.is_some_and(|p| p == neighbor) {
            continue;
        }

        if !resolve_dfs(neighbor, Some(node), adj_map, visited) {
            return false;
        }
    }

    true
}
