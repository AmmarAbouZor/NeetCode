// Bottom-up DP where order matters.
//
// dp[total] = number of ordered combinations that sum to `total`.
//
// For each total, try every number as the last picked value. If `num <= total`,
// then every way to build `total - num` can be extended by `num`:
//
// dp[total] += dp[total - num]
//
// dp[0] = 1 because there is one way to build sum 0: Pick nothing.
// This base case lets a single number equal to `total` contribute one way.
//
// Follow-up: if negative numbers are allowed, the count can be infinite
// because positive/negative cycles may keep the sum unchanged. A length limit
// would be needed, and the DP would need length as part of the state.
//
// Time: O(target * n) where n = nums.len()
// Space: O(target)

pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let target = target as usize;
    let mut dp = vec![0; target + 1];

    // There is one combination for zero total: Pick nothing.
    dp[0] = 1;

    for total in 1..=target {
        for &num in &nums {
            let num = num as usize;
            if num > total {
                continue;
            }

            dp[total] += dp[total - num];
        }
    }

    dp[target] as i32
}
