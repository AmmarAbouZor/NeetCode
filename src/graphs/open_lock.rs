use std::collections::{HashSet, VecDeque};

// BFS over an implicit graph for lock states.
//
// Each 4-digit code is a node. An edge exists between two codes if one wheel is
// turned one step forward and backward.
//
// Since every move has cost 1, BFS gives a minimum number of turns. The first
// time we pop the target from the queue, we have found the shortest path.
//
// Deadends are blocked states that can't be visited.
//
// Note: The tricky part is how to generate all neighbors with prev + next
// in the best efficient way.
//
// Time: O(10^4 * 4), because there are at most 10^4 states and each state
// generates 2 neighbors per digit.
// Space: O(10^4) for visited/deadends/queue.

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let deadends: HashSet<Vec<u8>> = deadends.into_iter().map(|d| d.into_bytes()).collect();
    let target = target.into_bytes();
    let start = vec![b'0'; 4];

    if deadends.contains(&start) {
        return -1;
    }

    let mut queue = VecDeque::with_capacity(1);

    queue.push_back((start.clone(), 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some((code, steps)) = queue.pop_front() {
        if code == target {
            return steps;
        }

        for next in get_neighbors(&code) {
            if visited.contains(&next) || deadends.contains(&next) {
                continue;
            }

            // Mark visited when adding to queue to avoid pushing the same
            // state multiple times.
            visited.insert(next.clone());
            queue.push_back((next, steps + 1));
        }
    }

    -1
}

// Generate all states reachable in one move.
// For each digit, turn it one step backward and one step forward, wrapping
// between 0 and 9.
fn get_neighbors(node: &[u8]) -> Vec<Vec<u8>> {
    let mut res = Vec::with_capacity(node.len().pow(2));

    for idx in 0..node.len() {
        let curr = node[idx] - b'0';

        // Add 9 before modulo to move backward and without underflow.
        let prev = (curr + 9) % 10;

        let next = (curr + 1) % 10;

        let mut prev_node = node.to_owned();
        prev_node[idx] = b'0' + prev;

        let mut next_node = node.to_owned();
        next_node[idx] = b'0' + next;

        res.extend([prev_node, next_node]);
    }

    res
}
