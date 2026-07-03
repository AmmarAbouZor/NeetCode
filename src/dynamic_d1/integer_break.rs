// Bottom-up DP.
//
// dp[num] = maximum product we can get from `num`, where `num` is allowed to
// stay unbroken when used as part of a larger split.
//
// That "allowed to stay unbroken" part is important. For example, when solving
// n = 10, a piece like 3 should be allowed to contribute product 3, not be
// forced to split into 1 * 2.
//
// For the original `n`, we must split it at least once, so initialize dp[n] to
// 0 instead of n. For smaller numbers, initialize dp[num] to num so they can be
// used as whole pieces.
//
// For every split:
// num = left + right
//
// The best product from the split is:
//
// dp[left] * dp[right]
//
// Time: O(n^2)
// Space: O(n)

pub fn integer_break(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let n = n as usize;
    let mut dp = vec![0_usize; n + 1];
    dp[1] = 1;

    for num in 2..=n {
        dp[num] = if num == n { 0 } else { num };
        for left in 1..num {
            let right = num - left;
            let val = dp[left] * dp[right];

            dp[num] = dp[num].max(val);
        }
    }

    dp[n] as i32
}
