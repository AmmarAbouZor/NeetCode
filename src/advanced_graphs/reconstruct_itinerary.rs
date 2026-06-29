use std::collections::HashMap;

// Hierholzer's algorithm for finding and Eulerian path.
//
// Each ticket is a directed edge from `from` to `to`. The problem guarantees hat
// all tickets can be used exactly once starting from "JFK", so we need an
// Eulerian path in this directed graph.
//
// Lexicographically smallest intinerary:
// Sort each adjacency list in descending order, then `pop()` from the end.
// This gives the smallest available destination in O(1) per edge removal.
//
// DFS consumes edges until it reaches and airport with no outgoing tickets left.
// Only then do we push that airport into `res`. This postorder push builds the
// intinerary in reverse, so we reverse the results at the end.
//
// Note:
// It's possible to use the same algorithm for graphs that can't be resolved.
// For that we just need to check the `res.len() == tickets.len() + 1`.
//
// Time: O(E logE) for sorting tickets withing adjacency list. then O(E) DFS.
// Space: O(E) for the graph and recursion stack/result.

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for ticket in tickets {
        let mut t = ticket.into_iter();
        let from = t.next().unwrap();
        let to = t.next().unwrap();

        graph.entry(from).or_default().push(to);
    }

    // Sort descending so pop() return the lexicographically smallest destination.
    for dists in graph.values_mut() {
        dists.sort_unstable_by(|a, b| b.cmp(a));
    }

    let mut res = Vec::new();

    dfs(String::from("JFK"), &mut graph, &mut res);

    res.reverse();

    res
}

// Consume all outgoing edges from `node`, then record `node` in postorder.
fn dfs(node: String, graph: &mut HashMap<String, Vec<String>>, res: &mut Vec<String>) {
    while let Some(next) = graph.get_mut(&node).and_then(|dest| dest.pop()) {
        dfs(next, graph, res);
    }

    res.push(node);
}
