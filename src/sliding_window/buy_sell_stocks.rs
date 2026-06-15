// Time O(N). Sliding window with two pointers going once.
// Space O(1). No allocations.

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut left = 0;
    let mut right = 1;

    let mut max = 0;
    while right < prices.len() {
        let profit = prices[right] - prices[left];

        if profit >= 0 {
            right += 1;
            max = max.max(profit);
        } else {
            left = right;
        }
    }

    max
}
