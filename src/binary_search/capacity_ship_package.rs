// Binary search on the minimum ship capacity.
//
// The answer must be between:
// - max(weights): every package must fit by itself
// - sum(weights): ship everything in one day
//
// For a candidate capacity, greedily simulate loading packages in order.
// Start a new day whenever adding the next package would exceed capacity.
//
// If the shipment can finish within `days`, the capacity is valid, so try a
// smaller one. Otherwise, the capacity is too small.
//
// Time: O(n * log(sum(weights) - max(weights)))
// Space: O(1)

pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut max = 0;
    let mut sum = 0;

    for &weight in &weights {
        max = max.max(weight);
        sum += weight;
    }

    let mut left = max;
    let mut right = sum;

    while left <= right {
        let mid = left + (right - left) / 2;
        if can_ship(&weights, days, mid) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    left
}

// Return true if all packages can be shipped within `days_limit` days using
// the given capacity.
//
// Greedy is valid because package order is fixed: each day should carry as many
// packages as possible before starting the next day.
fn can_ship(weights: &[i32], days_limit: i32, capacity: i32) -> bool {
    let mut days = 1;
    let mut sum = 0;

    for &weight in weights {
        if sum + weight > capacity {
            sum = weight;
            days += 1;
        } else {
            sum += weight;
        }

        if days > days_limit {
            return false;
        }
    }

    true
}
