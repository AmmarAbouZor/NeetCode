use std::{cmp::Reverse, collections::BinaryHeap};

// Sweep queries from smallest to largest.
//
// Sort intervals by start. For each query, add every interval whose start is
// <= query into a min-heap ordered by interval length.
//
// The heap may contain intervals that start before the query but already ended,
// so remove intervals with right < query before reading the answer.
//
// Heap stores (length, right). The smallest valid interval is at the top.
//
// Time: O(n logn + q logq + (n + q) log n): sort intervals + sort queries + heap
// Space: O(n + q)

pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|i| i[0]);

    let mut s_queries: Vec<_> = queries
        .iter()
        .enumerate()
        .map(|(idx, &n)| (n, idx))
        .collect();

    s_queries.sort_unstable_by_key(|&(query, _)| query);

    let mut res = vec![-1; queries.len()];

    let mut heap = BinaryHeap::new();
    let mut intervals = intervals.into_iter().peekable();

    for (query, idx) in s_queries {
        while intervals.peek().is_some_and(|i| i[0] <= query) {
            let interv = intervals.next().unwrap();
            let len = interv[1] - interv[0] + 1;
            heap.push(Reverse((len, interv[1])));
        }

        while let Some(Reverse((_len, right))) = heap.peek() {
            if *right < query {
                heap.pop();
            } else {
                break;
            }
        }

        if let Some(Reverse((len, _right))) = heap.peek() {
            res[idx] = *len;
        }
    }

    res
}
