// 2D Dynamic Programming by making a grid with the chars of two words as its columns and rows.
// Then the value for each cell is either:
// * 1 + diagonal value if chars are equal, because we extend the LCS of both previous prefixes.
// * max of left/current-row value and up/previous-row value if chars differ.

// DP Idea:
// dp[i][j] = LCS length using first i chars of text1 and first j chars of text2

// Note: Check the full grid version below before the optimized one.

// Time: O(n * m)
// Space: O(n): It can also be O(min(n, m)) if we picked the shorter word for grid columns.
pub fn longest_common_subsequence_optimized(text1: String, text2: String) -> i32 {
    let text1 = text1.as_bytes();
    let text2 = text2.as_bytes();
    let m = text1.len();
    let n = text2.len();

    let mut prev = vec![0; n + 1];

    for i in 1..=m {
        let mut cur = vec![0; n + 1];
        for j in 1..=n {
            cur[j] = if text1[i - 1] == text2[j - 1] {
                1 + prev[j - 1]
            } else {
                cur[j - 1].max(prev[j])
            };
        }

        prev = cur;
    }

    prev[n]
}

// Time: O(n * m)
// Space: O(n * m)
pub fn longest_common_subsequence_grid(text1: String, text2: String) -> i32 {
    let text1 = text1.as_bytes();
    let text2 = text2.as_bytes();

    let m = text1.len();
    let n = text2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if text1[i - 1] == text2[j - 1] {
                1 + dp[i - 1][j - 1]
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }

    dp[m][n]
}
