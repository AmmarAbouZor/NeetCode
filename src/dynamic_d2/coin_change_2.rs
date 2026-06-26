// DP State:
// dp[i][a] = number of ways to make amount a
//            using the first i coins

// Transitions: We have two choices:
// * Skip current coin: dp[i - 1][a]
// * Include current coin if valid: dp[i][a - coin] where a >= coin

// Note:
// Include uses dp[i][a - coin], not dp[i - 1][a - coin],
// because the same coin can be used multiple times.

// Time: O(n * a) where n is coins length and a is amount.
// Space: O(n * a)
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let n = coins.len();
    let amount = amount as usize;

    if amount == 0 {
        return 1;
    }

    if n == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; amount + 1]; n + 1];

    // Base case:
    // There is one way to make 0 amount: Choose no coins
    for row in dp.iter_mut() {
        row[0] = 1;
    }

    for c in 1..=n {
        for a in 1..=amount {
            let skip_comb = dp[c - 1][a];
            let include_comb = a
                .checked_sub(coins[c - 1] as usize)
                .map(|r| dp[c][r])
                .unwrap_or_default();
            dp[c][a] = skip_comb + include_comb;
        }
    }

    dp[n][amount]
}
