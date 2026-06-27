use std::{cmp::Reverse, collections::BinaryHeap};

// Shortest paths in a weighted graph with positive weights => Dijkstra.
// Use vectors for graph/dist because node labels are in range 1..=n.
//
// Each edge can be relaxed, and each successful relaxation pushes a candidate
// distance into the min-heap. Heap push/pop costs O(log V).
//
// Given V = number of nodes and E = number of edges:
// Time: O((V + E) log V), often written as O(E log V).
// Space: O(V + E), for graph, distances, and heap.

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let start = k as usize;

    let mut graph = vec![Vec::new(); n + 1];
    for node in times {
        let u = node[0] as usize;
        let v = node[1] as usize;
        let w = node[2];

        graph[u].push((v, w));
    }

    let mut dist = vec![i32::MAX; n + 1];
    dist[start] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((time, node))) = heap.pop() {
        if time > dist[node] {
            continue;
        }

        for &(next, weight) in &graph[node] {
            let next_time = time + weight;
            if next_time < dist[next] {
                dist[next] = next_time;
                heap.push(Reverse((next_time, next)));
            }
        }
    }

    let mut res = 0;

    // Input range is 1..=n means that we need to skip dist[0]
    for d in dist.into_iter().skip(1) {
        // Check if any node isn't reached.
        if d == i32::MAX {
            return -1;
        }

        res = res.max(d);
    }

    res
}
