// DP solution with optimization by avoiding saving the whole dp array as
// we always need the last two numbers only.
// We keep track on the last two numbers and keep moving.

// Time O(n) we still visited each node
// Space O(1) we need two variables only.

pub fn climb_stairs_dp_optmized(n: i32) -> i32 {
    let n = n as usize;

    if n <= 2 {
        return n as i32;
    }

    let mut prev1 = 2;
    let mut prev2 = 1;
    for _ in 3..=n {
        let cur = prev1 + prev2;
        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}

// Standard DP solution creating the DP vector and filling it from bottom up
// Time: O(n) we visited each node once
// Space: O(n) as we are creating the whole dp here.
pub fn climb_stairs_dp(n: i32) -> i32 {
    let n = n as usize;

    if n <= 2 {
        return n as i32;
    }

    let mut dp = vec![0; n + 1];

    dp[1] = 1;
    dp[2] = 2;
    for i in 3..=n {
        dp[i] = dp[i - 2] + dp[i - 1];
    }

    dp[n]
}
