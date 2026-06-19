// Optimized DP version using two variables instead of a full DP array.
// Time: O(n)
// Space: O(1)

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut prev1 = 0; // dp[i -1]
    let mut prev2 = 0; // dp[i -2]

    for i in 2..=n {
        let cur = (prev1 + cost[i - 1]).min(prev2 + cost[i - 2]);
        prev2 = prev1;
        prev1 = cur;
    }
    prev1
}

// Dynamic programming bottom up using dp array.
// The main idea that we can reach n with either n - 1 or n -2 so we pick
// the min from them. Then each one of them can also be reached with the minimum of
// n1 - 1 and n1 - 2 so we can fill the dp using the minimum of the two decisions

// Another explain:
// Bottom-up DP using a full dp array.
// dp[i] is the minimum cost to reach step i.
// To reach step i, we either came from i - 1 and paid cost[i - 1],
// or came from i - 2 and paid cost[i - 2].

// Time O(n)
// Space O(n) for dp array.
pub fn min_cost_climbing_stairs_dp(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = vec![0; n + 1];

    for i in 2..=n {
        dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
    }

    dp[n]
}
