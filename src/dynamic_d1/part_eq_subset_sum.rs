// 0/1 knapsack DP.
//
// We need to know whether some subset sums to total / 2.
// If total is odd, equal partition is impossible.
//
// dp[sum] = true if we can build `sum` using the numbers processed so far.
//
// For each num, update sums in reverse so num is used at most once.
// Example: if num = 2 and we iterate forward:
// - dp[2] becomes true from dp[0]
// - then dp[4] becomes true from dp[2]
// That incorrectly uses the same 2 twice in one pass.
//
// Reverse iteration avoids this because dp[sum - num] still belongs to the
// previous state, before the current num was added.
//
// Time: O(n * target), where target = sum(nums) / 2.
// Space: O(target).

pub fn can_partition(nums: Vec<i32>) -> bool {
    let total: i32 = nums.iter().sum();

    if total % 2 != 0 {
        return false;
    }

    let target = (total / 2) as usize;

    let mut dp = vec![false; target + 1];

    dp[0] = true;

    for num in nums {
        let num = num as usize;
        for sum in (num..=target).rev() {
            if dp[sum - num] {
                dp[sum] = true;
            }
        }
    }

    dp[target]
}
