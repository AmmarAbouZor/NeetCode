// Keep the code as simple as possible because the algorithm is complex enough.
#![allow(clippy::needless_range_loop)]

// dp[i][j] = minimum operations to convert word1[0..i] into word2[0..j].
//
// Base cases:
// - dp[i][0] = i, delete all i chars from word1
// - dp[0][j] = j, insert all j chars from word2
//
// If last chars match, no new operation is needed:
// dp[i][j] = dp[i - 1][j - 1]
//
// If last chars differ, think about the final operation:
// - insert word2[j - 1]: first convert word1[0..i] into word2[0..j - 1]
// - delete word1[i - 1]: first convert word1[0..i - 1] into word2[0..j]
// - replace last char: first convert word1[0..i - 1] into word2[0..j - 1]
//
// Time: O(m * n)
// Space: O(m * n)
pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();

    let m = word1.len();
    let n = word2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Fill Base Cases first
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if word1[i - 1] == word2[j - 1] {
                dp[i - 1][j - 1]
            } else {
                let insert = dp[i][j - 1];
                let delete = dp[i - 1][j];
                let replace = dp[i - 1][j - 1];

                1 + insert.min(delete).min(replace)
            };
        }
    }

    dp[m][n] as i32
}
