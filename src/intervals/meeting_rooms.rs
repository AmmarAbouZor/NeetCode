// Sort intervals by start time, then check adjacent intervals for overlap.

// Time: O(N*LogN) for sorting. O(N) for iteration.
// Space: O(1), except space needed by sorting.

pub struct Interval {
    start: i32,
    end: i32,
}

// Same as standard below but it uses windows() function on vectors
pub fn can_attend_meetings_idiomatic(intervals: Vec<Interval>) -> bool {
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|i| i.start);
    for win in intervals.windows(2) {
        if win[1].start < win[0].end {
            return false;
        }
    }

    true
}

// Standard straight forward solution
pub fn can_attend_meetings_simple(intervals: Vec<Interval>) -> bool {
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|i| i.start);

    for i in 1..intervals.len() {
        if intervals[i].start < intervals[i - 1].end {
            return false;
        }
    }

    true
}

pub fn can_attend_meetings(intervals: Vec<Interval>) -> bool {
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|i| i.start);

    let mut intervals = intervals.into_iter();

    let Some(first) = intervals.next() else {
        return true;
    };

    let mut prev_end = first.end;

    for interval in intervals {
        if interval.start < prev_end {
            return false;
        }

        prev_end = interval.end;
    }

    true
}
