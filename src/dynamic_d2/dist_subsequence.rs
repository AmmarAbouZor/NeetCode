// NOTE: The same problem is solved with DP in two ways:
// * DP bottom-up
// * DP top-down with DFS and memoization.
// It's a good use case to understand how the same dp idea applies on both of them.

// Bottom-up DP using prefix lengths.
//
// dp[s_idx][t_idx] = number of ways to form t[0..t_idx]
// using chars from s[0..s_idx].
//
// The row index is how many chars of `s` are available.
// The column index is how many chars of `t` we still need to form.
//
// Base case:
// dp[s_idx][0] = 1 because empty `t` can be formed from any prefix of `s`
// by choosing no chars.
//
// Transition:
// For current chars s[s_idx - 1] and t[t_idx - 1]:
// - always skip current s char: dp[s_idx - 1][t_idx]
// - if chars match, also use it: dp[s_idx - 1][t_idx - 1]
//
// Time: O(s.len() * t.len())
// Space: O(s.len() * t.len())

pub fn nums_distinct_dp(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let s_len = s.len();
    let t_len = t.len();

    let mut dp = vec![vec![0; t_len + 1]; s_len + 1];

    // Empty t can always be formed in one way: choose no chars.
    for s_idx in 0..=s_len {
        dp[s_idx][0] = 1;
    }

    for s_idx in 1..=s_len {
        for t_idx in 1..=t_len {
            dp[s_idx][t_idx] = if s[s_idx - 1] == t[t_idx - 1] {
                let skip = dp[s_idx - 1][t_idx];
                let include = dp[s_idx - 1][t_idx - 1];
                skip + include
            } else {
                // Only skip on no match
                dp[s_idx - 1][t_idx]
            }
        }
    }

    dp[s_len][t_len]
}

// Top-down DP with DFS + memoization.
//
// dfs(s_idx, t_idx) = number of ways to form t[t_idx..] from s[s_idx..].
//
// If s[s_idx] == t[t_idx], we have two choices:
// - skip s[s_idx]
// - use s[s_idx] to match t[t_idx]
//
// If they do not match, we can only skip s[s_idx].
//
// Base cases:
// - matched all of t => 1 valid subsequence
// - ran out of s before matching t => 0
//
// Time: O(s.len() * t.len()), because each state is computed once.
// Space: O(s.len() * t.len()) for memo, plus O(s.len()) recursion stack.

pub fn nums_distinct_memo(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut dp = vec![vec![-1; t.len()]; s.len()];

    dfs_memo(0, 0, s, t, &mut dp)
}

fn dfs_memo(s_idx: usize, t_idx: usize, s: &[u8], t: &[u8], dp: &mut [Vec<i32>]) -> i32 {
    // matched all target. return 1
    if t_idx == t.len() {
        return 1;
    }

    // No source chars left to match target. return 0
    if s_idx == s.len() {
        return 0;
    }

    if dp[s_idx][t_idx] != -1 {
        return dp[s_idx][t_idx];
    }

    let res = if s[s_idx] == t[t_idx] {
        // On match we can either skip it or proceed with the match.
        let skip = dfs_memo(s_idx + 1, t_idx, s, t, dp);
        let include = dfs_memo(s_idx + 1, t_idx + 1, s, t, dp);
        skip + include
    } else {
        // Skip only here.
        dfs_memo(s_idx + 1, t_idx, s, t, dp)
    };

    dp[s_idx][t_idx] = res;

    res
}
