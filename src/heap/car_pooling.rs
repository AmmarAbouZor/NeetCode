use std::{cmp::Reverse, collections::BinaryHeap};

// Difference array with coordinate offset.
//
// Each trip adds passengers at `start` and removes them at `end`. A prefix sum
// over these changes gives the number of passengers in the car at each point.
//
// This version sizes the timeline from min start to max end instead of relying
// on fixed LeetCode bounds.
//
// Time: O(n + R), where R = max_end - min_start + 1.
// Space: O(R).
pub fn car_pooling_sweep_optimized(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    if trips.is_empty() {
        return true;
    }

    let (mut total_start, mut total_end) = (i32::MAX, i32::MIN);
    for trip in &trips {
        let start = trip[1];
        let end = trip[2];

        total_start = total_start.min(start);
        total_end = total_end.max(end);
    }

    let pref_len = (total_end - total_start + 1) as usize;
    let pref_offset = total_start as usize;

    let mut timeline = vec![0_i32; pref_len];

    for trip in &trips {
        let (t_count, start, end) = (trip[0], trip[1], trip[2]);

        let start_idx = start as usize - pref_offset;
        timeline[start_idx] += t_count;

        let end_idx = end as usize - pref_offset;
        timeline[end_idx] -= t_count;
    }

    let mut pass_count = 0;

    for count_event in timeline {
        pass_count += count_event;
        if pass_count > capacity {
            return false;
        }
    }

    true
}

// Difference array using the fixed LeetCode location bound.
//
// Locations are in 0..=1000, so a fixed timeline is the simplest version.
// Pickups add passengers at `start`; dropoffs subtract passengers at `end`.
//
// Time: O(n + 1000), effectively O(n).
// Space: O(1000), effectively O(1).
pub fn car_pooling_leetcode_constrains(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut timeline = vec![0; 1001];

    for trip in trips {
        let passengers = trip[0];
        let start = trip[1] as usize;
        let end = trip[2] as usize;

        timeline[start] += passengers;
        timeline[end] -= passengers;
    }

    let mut current = 0;

    for change in timeline {
        current += change;

        if current > capacity {
            return false;
        }
    }

    true
}

// Event sweep.
//
// Turn every trip into two events:
// - pickup:  (start, +passengers)
// - dropoff: (end, -passengers)
//
// Sort events by location and apply them in order. At the same location,
// dropoffs are negative, so tuple sorting applies them before pickups.
//
// This is a good general solution when locations are not bounded.
//
// Time: O(n log n), for sorting 2n events.
// Space: O(n).
pub fn car_pooling_sweep_simple(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut time_line = Vec::with_capacity(trips.len() * 2);

    for trip in trips {
        let (in_pass, start, end) = (trip[0], trip[1], trip[2]);
        time_line.push((start, in_pass));
        time_line.push((end, -in_pass));
    }

    time_line.sort_unstable();

    let mut pass_count = 0;

    for (_time, event_count) in time_line {
        pass_count += event_count;

        if pass_count > capacity {
            return false;
        }
    }

    true
}

// Min-heap simulation.
//
// Process trips by start location. The heap stores active trips as
// `(end_location, passengers)`, ordered by earliest dropoff.
//
// Before starting a trip, remove every active trip whose end <= current start.
// Then add the new passengers and check capacity.
//
// This is similar to Meeting Rooms and is useful when trips are processed in
// start order and active intervals need to expire dynamically.
//
// Time: O(n log n), for sorting trips and heap operations.
// Space: O(n).
pub fn car_pooling_heap(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut trips = trips;

    trips.sort_unstable_by_key(|trip| trip[1]);

    let mut pass: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

    let mut passengers = 0_i32;

    for trip in trips {
        let (in_pass, start, end) = (trip[0], trip[1], trip[2]);

        while let Some(&Reverse((out_time, out_pass))) = pass.peek() {
            if out_time > start {
                break;
            }

            pass.pop();
            passengers -= out_pass;
        }

        passengers += in_pass;

        if passengers > capacity {
            return false;
        }

        pass.push(Reverse((end, in_pass)));
    }

    true
}
