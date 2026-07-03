use std::{cmp::Reverse, collections::BinaryHeap};

// Minimum Spanning Tree Problem which can be solved with Prim's algorithm.
// Start with the heap solution below as it's simpler to understand.

// This solution avoids the heap because we are working on complete graph aka
// every node has an edge to every other node
// min_dist[i] = cheapest known cost to connect point i to the current MST
// At each step, choose the unvisited point with the smallest min_dist,
// then use it to update the connection cost for every other unvisited point.

// Time: O(N^2)
// Space: O(N)
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();

    if n <= 1 {
        return 0;
    }

    let mut visited = vec![false; n];
    let mut min_dist = vec![i32::MAX; n];

    min_dist[0] = 0;

    let mut total_cost = 0;

    for _ in 0..n {
        let point = (0..n)
            .filter(|&i| !visited[i])
            .min_by_key(|&i| min_dist[i])
            .expect("There is always at least one unvisited point in each Prim step");

        visited[point] = true;
        total_cost += min_dist[point];

        let cur = &points[point];
        for (idx, next) in points.iter().enumerate() {
            if visited[idx] {
                continue;
            }

            let next_dist = (cur[0] - next[0]).abs() + (cur[1] - next[1]).abs();
            min_dist[idx] = min_dist[idx].min(next_dist);
        }
    }

    total_cost
}

// We use heap to ensure we are getting the shortest distance for the unvisited nodes.
// Keeping a count of visited points makes it easy to stop even if the heap is still having items.

// Time: O(N^2 LogN) as we may push edges to all other points
// Space: O(N^2) worst case scenario
pub fn min_cost_connect_points_heap(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();

    if n <= 1 {
        return 0;
    }

    let mut visited = vec![false; n];
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, 0)));

    let mut total_cost = 0;
    let mut visited_count = 0;

    while visited_count < n {
        let Reverse((cost, point)) = heap
            .pop()
            .expect("Heap can't be empty while we have unvisited points");
        if visited[point] {
            continue;
        }

        visited[point] = true;
        visited_count += 1;
        total_cost += cost;

        let cur = &points[point];
        for (idx, next) in points.iter().enumerate() {
            if visited[idx] {
                continue;
            }

            let dist = (cur[0] - next[0]).abs() + (cur[1] - next[1]).abs();

            heap.push(Reverse((dist, idx)));
        }
    }

    total_cost
}
