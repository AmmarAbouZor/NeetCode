// Bottom-up DP.
//
// dp[num] = minimum number of perfect squares needed to sum to `num`.
//
// Precompute all square numbers <= n. For each num, try every square that can.
// be used as the last picked value:
//
// dp[num] = min(dp[num], 1 + dp[num - square])
//
// dp[0] = 0 because zero needs no numbers.
//
// Time: O(n * sqrt(n))
// Space: O(n + sqrt(n)) for dp and the squares list.

pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;

    let mut squares = Vec::new();

    let mut x = 1;

    while x * x <= n {
        squares.push(x * x);
        x += 1;
    }

    // Worst case is using 1^2 exactly n times, so n + 1 is safely impossible.
    let inf = n as i32 + 1;
    let mut dp = vec![inf; n + 1];

    dp[0] = 0;

    for num in 1..=n {
        for &square in squares.iter() {
            if square > num {
                break;
            }
            dp[num] = dp[num].min(1 + dp[num - square]);
        }
    }

    dp[n]
}
