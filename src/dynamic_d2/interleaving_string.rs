// DP State:
// dp[i][j] = true if s3[..i + j] can be formed
//            using s1[..i] and s2[..j]
//
// To decide dp[i][j], think about the last character used to build s3[..i + j].
// It must come from either:
//
// 1. s1[i - 1]
//    Then the previous state must be dp[i - 1][j], and s1[i - 1]
//    must match the last needed char in s3: s3[i + j - 1].
//
// 2. s2[j - 1]
//    Then the previous state must be dp[i][j - 1], and s2[j - 1]
//    must match the last needed char in s3: s3[i + j - 1].
//
// If either option works, dp[i][j] is true.
//
// dp[0][0] = true because empty s1 + empty s2 forms empty s3.
// The first row and first column are handled naturally by the same transitions.
//
// Time: O(m * n), where m = s1.len() and n = s2.len().
// Space: O(m * n).

pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let s3 = s3.as_bytes();

    let m = s1.len();
    let n = s2.len();

    if m + n != s3.len() {
        return false;
    }

    let mut dp = vec![vec![false; n + 1]; m + 1];
    // Empty + Empty forms Empty
    dp[0][0] = true;

    for i in 0..=m {
        for j in 0..=n {
            if i > 0 && dp[i - 1][j] && s1[i - 1] == s3[i + j - 1] {
                dp[i][j] = true;
            }

            if j > 0 && dp[i][j - 1] && s2[j - 1] == s3[i + j - 1] {
                dp[i][j] = true;
            }
        }
    }

    dp[m][n]
}
