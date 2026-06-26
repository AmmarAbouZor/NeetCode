// DP State:
// hold[i] = max profit at end of day i while holding a stock
// not_hold[i] = max profit at end of day i while not holding a stock

// Transition:
// hold[i] = max(
//     hold[i - 1],              // keep holding
//     not_hold[i - 2] - prices[i]   // buy today after cooldown
// )
// not_hold[i] = max(
//     not_hold[i - 1],              // keep not holding
//     hold[i - 1] + prices[i]   // sell today
// )

// Return not_hold[last], because holding an unsold stock cannot increase final profit.

// It's possible to reduce space using variables but this is better to understand.

// Time: O(n)
// Space: O(n)
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let n = prices.len();

    let mut hold = vec![0; n];
    let mut not_hold = vec![0; n];

    hold[0] = -prices[0];

    for idx in 1..n {
        let not_hold_prev = idx.checked_sub(2).map(|i| not_hold[i]).unwrap_or_default();

        hold[idx] = hold[idx - 1].max(not_hold_prev - prices[idx]);
        not_hold[idx] = not_hold[idx - 1].max(hold[idx - 1] + prices[idx]);
    }

    not_hold[n - 1]
}
