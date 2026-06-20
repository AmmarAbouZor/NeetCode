// We go through intervals in linear loop and have a separate results vector.
// The important start point here is to have a separate result array which keeps
// the three cases simple.

// Time: O(n) linear iteration
// Space: O(n) for results array. O(1) excluding the results

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    // Allocate for the worst case upfront instead of letting results grow dynamically.
    let mut results = Vec::with_capacity(intervals.len() + 1);

    let mut start = new_interval[0];
    let mut end = new_interval[1];
    let mut inserted = false;

    for interval in intervals {
        let cur_start = interval[0];
        let cur_end = interval[1];

        if cur_end < start {
            // Current interval is completely before new interval
            results.push(interval);
        } else if cur_start > end {
            // Current interval is completely after the new interval.
            // At the first time we hit this point then we need to insert new interval
            // As it should be also merged at this point if needed.
            if !inserted {
                results.push(vec![start, end]);
                inserted = true;
            }

            results.push(interval);
        } else {
            // Overlapping interval merge.
            start = start.min(cur_start);
            end = end.max(cur_end);
        }
    }

    // If the merged new interval belongs at the end, it has not been inserted yet.
    if !inserted {
        results.push(vec![start, end]);
    }

    results
}

// Same concept as the one above. It just uses iterator methods to return early.
// This shouldn't be used in an interview as it's harder to follow and debug and
// easier to get wrong.
pub fn insert_tricky_iterator(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    // Allocating for worst case upfront as it will still better than allocate
    // and move memory multiple time.
    let mut result = Vec::with_capacity(intervals.len() + 1);

    let mut start = new_interval[0];
    let mut end = new_interval[1];
    let mut intervals = intervals.into_iter();

    for interval in intervals.by_ref() {
        let cur_start = interval[0];
        let cur_end = interval[1];
        if cur_end < start {
            result.push(interval);
        } else if cur_start > end {
            // At first time it's completely after the interval we can
            // Add the interval and append the rest and return early.
            result.push(vec![start, end]);
            result.push(interval);
            result.extend(intervals);
            return result;
        } else {
            start = start.min(cur_start);
            end = end.max(cur_end);
        }
    }

    // Case if intervals (original or updated) comes at the end of
    // the intervals array, then it should be added.
    result.push(vec![start, end]);

    result
}
