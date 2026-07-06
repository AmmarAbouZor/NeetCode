// Top-down interval DP with memoization.
//
// Key mental model:
// Do not choose which balloon to burst first.
// Choose which balloon is burst last inside an interval.
//
// Add virtual balloons with value 1 at both ends:
//
// vals = [1] + nums + [1]
//
// dfs(left, right) = max coins we can get by bursting all balloons strictly
// between `left` and `right`.
//
// `left` and `right` are boundary balloons and are not burst in this subproblem.
// If `mid` is burst last inside (left, right), then every other balloon in the
// interval is already gone, so `mid`'s final neighbors are exactly left and right.
//
// coins = dfs(left, mid) + vals[left] * vals[mid] * vals[right] + dfs(mid, right)
//
// Try every possible `mid` as the last balloon and memoize the best result.
//
// Time: O(n^3): O(n^2) intervals, and each interval tries O(n) choices.
// Space: O(n^2) for memo plus recursion stack.
pub fn max_coins_memo(nums: Vec<i32>) -> i32 {
    let mut vals = Vec::with_capacity(nums.len() + 2);
    vals.push(1);
    vals.extend(nums);
    vals.push(1);

    let n = vals.len();
    let mut memo = vec![vec![-1; n]; n];

    dfs(0, n - 1, &vals, &mut memo)
}

fn dfs(left: usize, right: usize, vals: &[i32], memo: &mut [Vec<i32>]) -> i32 {
    // No balloon strictly between the boundaries.
    if right == left + 1 {
        return 0;
    }

    if memo[left][right] != -1 {
        return memo[left][right];
    }

    let mut best = 0;

    // `mid` is the last balloon to burst in this interval.
    for mid in left + 1..right {
        let coins = dfs(left, mid, vals, memo)
            + vals[left] * vals[mid] * vals[right]
            + dfs(mid, right, vals, memo);

        best = best.max(coins);
    }

    memo[left][right] = best;
    best
}

// Bottom-up interval DP.
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
