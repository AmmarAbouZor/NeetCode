// Using DFS is straightforward here:
// * Create an adjacency list from undirected graph edges.
// * Keep track of visited nodes.
// * Iterate through every node. Each unvisited node starts a new connected component.

// Time: O(V + E), because each node is visited once and each undirected edge
// is considered from both endpoints.
// Space: O(V + E), for the adjacency list, visited array, and recursion stack.

pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];

    for pair in edges {
        let (n1, n2) = (pair[0] as usize, pair[1] as usize);
        adj[n1].push(n2);
        adj[n2].push(n1);
    }

    let mut visited = vec![false; n];

    let mut count = 0;
    for node in 0..n {
        if visited[node] {
            continue;
        }
        dfs(node, &adj, &mut visited);
        count += 1;
    }

    count
}

fn dfs(node: usize, adj: &[Vec<usize>], visited: &mut [bool]) {
    visited[node] = true;

    for &child in &adj[node] {
        if visited[child] {
            continue;
        }

        dfs(child, adj, visited);
    }
}
