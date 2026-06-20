// Similar to meeting room 1 but keep room end times in a min-heap, so the room that
// frees earliest is on top.

// Time: O(n log n + n log m), where m is the number of rooms.
// Since m <= n, this is O(n log n).
// Space: O(m) for the heap, excluding sorting internals.

pub struct Interval {
    start: i32,
    end: i32,
}

pub fn min_meeting_rooms_heap(intervals: Vec<Interval>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|i| i.start);
    use std::{cmp::Reverse, collections::BinaryHeap};

    let mut rooms: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for interval in intervals {
        if rooms.peek().is_some_and(|r| r.0 <= interval.start) {
            rooms.pop();
        }
        rooms.push(Reverse(interval.end));
    }

    rooms.len() as i32
}

// Similar to meeting room 1 but keep the rooms in vector and iterate through it
// for each interval.

// Time: O(N*LogN + N*M) where N is intervals count and M is the min meeting rooms
// Space: O(M) vector + O(N*LogN) as space needed for sorting.
pub fn min_meeting_rooms_linear(intervals: Vec<Interval>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|i| i.start);

    let mut rooms: Vec<i32> = Vec::new();

    for interval in intervals {
        if let Some(room) = rooms.iter_mut().find(|r| **r <= interval.start) {
            *room = interval.end;
        } else {
            rooms.push(interval.end);
        }
    }

    rooms.len() as i32
}
