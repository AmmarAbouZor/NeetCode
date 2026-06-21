//! Main Concepts:
//! * Sort at the start as vehicle can never go pass each others
//! * Go from the end and compare times to reach the targets since it's not needed
//!   to know when exactly two cars make a fleet. We just need to know if they will be a fleet at the end.
//! * A stack is not needed for this solution as we need to keep track on the latest fleet only.

// Time: O(N*logN) for sorting.
// SpaceL O(1) + Space needed for sorting
pub fn car_fleet_no_stack(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();

    cars.sort_unstable();

    let mut fleets = 0;
    let mut slowest_time = 0.0;

    for (pos, speed) in cars.into_iter().rev() {
        let time = (target - pos) as f32 / speed as f32;

        if time > slowest_time {
            fleets += 1;
            slowest_time = time;
        }
    }

    fleets
}

pub fn car_fleet_stack(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();

    cars.sort_unstable();

    let mut fleet_times: Vec<f32> = Vec::new();

    for (pos, speed) in cars.into_iter().rev() {
        let time = (target - pos) as f32 / speed as f32;

        if let Some(&last_time) = fleet_times.last() {
            if time > last_time {
                fleet_times.push(time);
            }
        } else {
            fleet_times.push(time);
        }
    }

    fleet_times.len() as i32
}
