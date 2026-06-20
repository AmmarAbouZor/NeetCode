// Solution becomes much easier once we sort the input.
// In one run we start with the first item and keep track on the end of previous
// interval.
// On overlap, remove one interval and keep the smaller end, because the shorter
// ending interval is less likely to overlap future intervals.

// Note: 1-3 and 3-5 aren't overlapping

// Time: O(n log n) for sorting, plus O(n) scan.
// Space: O(1) excluding sort implementation details.

pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_unstable();

    let mut intervals = intervals.into_iter();

    let Some(first) = intervals.next() else {
        return 0;
    };

    let mut last_end = first[1];

    let mut count = 0;

    for interval in intervals {
        let start = interval[0];
        let end = interval[1];
        if start >= last_end {
            last_end = end;
        } else {
            count += 1;
            last_end = last_end.min(end);
        }
    }

    count
}
