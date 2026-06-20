// Merge intervals with sorting at start
// We start with results with first interval and iterate through the rest:
// At each step, compare the current interval with the last merged interval:
// If true we extend the last interval length if needed, otherwise we add the interval.

// Time: O(N*LogN) because of sorting at the start. The iteration of intervals is O(n).
// Space: O(N) for the output.

pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by_key(|i| i[0]);

    let mut intervals = intervals.into_iter();

    let Some(first) = intervals.next() else {
        return Vec::new();
    };

    let mut results = vec![first];

    for interval in intervals {
        let start = interval[0];
        let end = interval[1];
        let last = results.last_mut().expect("results has first interval");

        if start <= last[1] {
            last[1] = last[1].max(end)
        } else {
            results.push(interval);
        }
    }

    results
}
