// Interval DP
//
// Key mental model:
// Do not choose which balloon to burst first.
// Choose which balloon is burst last inside an interval.
//
// Add virtual balloons with value 1 at both ends:
//
// vals = [1] + nums + [1]
//
// These boundary balloons are never burst. They let every real balloon have a
// left and right neighbors when it is the last balloon to burst in some interval.
//
// dp[left][right] = max coins we can get by bursting all balloons strictly
// between `left` and `right`.
//
// So `left` and `right` are boundary indices, not balloons we burst in this subproblem.
//
// If `mid` is the last balloon burst between `left` and `right`, then by that
// time every balloon between left..mid and mid..right is already gone.
// Therefore the neighbors of `mid` are exactly `left` and `right`, giving:
//
// coins from bursting mid last = vals[left] * vals[mid] * vals[right]
//
// Total for choosing `mid` last:
//
// dp[left][mid] + vals[left] * vals[mid] * vals[right] + dp[mid][right]
//
// Try every possible `mid` and take the max.
//
// Fill smaller intervals first. `gap` is the distance between left and right.
// We need at least one real balloon between them, so gap starts at 2.
//
// Time: O(n^3): O(n^2) intervals, and each interval tries O(n) choices.
// Space: O(n^2) for dp.

pub fn max_coins(nums: Vec<i32>) -> i32 {
    let mut vals = Vec::with_capacity(nums.len() + 2);
    vals.push(1);
    vals.extend(nums);
    vals.push(1);

    let m = vals.len();
    let mut dp = vec![vec![0; m]; m];

    // `gap` is the distance between left and right.
    // We need at least one balloon between them, so gap starts at 2.
    for gap in 2..m {
        for left in 0..m - gap {
            let right = left + gap;

            // mid is the last balloon to burst in this interval.
            for mid in left + 1..right {
                let coins = dp[left][mid] + vals[left] * vals[mid] * vals[right] + dp[mid][right];
                dp[left][right] = dp[left][right].max(coins);
            }
        }
    }

    dp[0][m - 1]
}
