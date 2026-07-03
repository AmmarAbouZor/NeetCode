// Bottom-up DP.
//
// dp[i] = the i-th Tribonacci number.
//
// Base:
// T0 = 0, T1 = 1, T2 = 1
//
// Recurrence:
// Tn = Tn-1 + Tn-2 + Tn-3
//
// Time: O(n)
// Space: O(n)
pub fn tribonacci(n: i32) -> i32 {
    let n = n as usize;

    match n {
        0 => return 0,
        1 => return 1,
        _ => {}
    }
    let mut dp = vec![0; n + 1];

    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 1;

    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
    }

    dp[n]
}

// Space-optimized DP.
//
// Keep only the last three states:
// prev1 = dp[i - 1]
// prev2 = dp[i - 2]
// prev3 = dp[i - 3]
//
// Time: O(n)
// Space: O(1)
pub fn tribonacci_optimized(n: i32) -> i32 {
    let n = n as usize;

    match n {
        0 => return 0,
        1 | 2 => return 1,
        _ => {}
    }

    let mut prev3 = 0;
    let mut prev2 = 1;
    let mut prev1 = 1;

    for _ in 3..=n {
        let cur = prev1 + prev2 + prev3;
        prev3 = prev2;
        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}
