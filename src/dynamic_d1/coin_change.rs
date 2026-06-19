// Bottom-up DP.
// dp[x] is the minimum number of coins needed to make amount x.
// For each amount x, try every coin. If coin <= x, then one candidate is:
// dp[x - coin] + 1.
//
// We initialize unreachable states with amount + 1 because no valid answer can
// need more than amount coins. This avoids overflow from using i32::MAX and then
// adding 1.
//
// Time: O(amount * c), where c is coins.len().
// Space: O(amount), for dp.

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let impossible = amount + 1;
    let amount = amount as usize;
    let mut dp = vec![impossible; amount + 1];
    dp[0] = 0;

    for a in 1..=amount {
        for &c in &coins {
            let c = c as usize;
            if c <= a {
                dp[a] = dp[a].min(dp[a - c] + 1);
            }
        }
    }

    let answer = dp[amount];
    if answer == impossible { -1 } else { answer }
}
