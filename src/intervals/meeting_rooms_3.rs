use std::{cmp::Reverse, collections::BinaryHeap};

// Two-heap simulation.
//
// `available` stores free room ids, ordered by smallest room id.
// `busy` stores occupied rooms as (free_time, room_id), ordered by earliest
// free time, then smallest room id.
//
// For each meeting in start-time order:
// - release every room with free_time <= meeting start
// - if a room is available, use the smallest room id
// - otherwise delay the meeting until the earliest busy room becomes free
//
// When delayed, keep the original meeting duration and set:
//
// new_end = earliest_free_time + duration
//
// At the end, return the room with the largest meeting count. The final scan
// only updates on strictly greater count, so ties keep the smaller room id.
//
// Time: O(m log m + m log n), for sorting meetings and heap operations, where m = meetings.len().
// Space: O(n) for the heaps and usage counts.

pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut meetings = meetings;
    meetings.sort_unstable();

    let mut available: BinaryHeap<Reverse<usize>> =
        (0..n).map(|num| Reverse(num as usize)).collect();

    let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

    let mut used = vec![0; n];

    for meet in meetings {
        let start = meet[0] as i64;
        let end = meet[1] as i64;

        // Rooms that are free by this meeting's start become available again.
        while let Some(&Reverse((free_time, room))) = busy.peek() {
            if free_time > start {
                break;
            }

            busy.pop();
            available.push(Reverse(room));
        }

        if let Some(Reverse(room)) = available.pop() {
            used[room] += 1;
            busy.push(Reverse((end, room)));
        } else {
            // No room is free, so delay this meeting to the earliest available room.
            let Reverse((free_time, room)) = busy
                .pop()
                .expect("busy must have items if available is empty");
            let duration = end - start;
            let new_end = free_time + duration;

            used[room] += 1;
            busy.push(Reverse((new_end, room)));
        }
    }

    let mut best_idx = 0;
    for idx in 1..n {
        if used[idx] > used[best_idx] {
            best_idx = idx;
        }
    }

    best_idx as i32
}
