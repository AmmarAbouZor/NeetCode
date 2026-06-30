// Bottom-up DP over prefixes
// NOTE: Check the Top-down solution below
//
// dp[s_idx][p_idx] = whether s[0..s_idx] matches p[0..p_idx].
//
// `s_idx` and `p_idx` are prefix lengths, so the current chars are:
// - s[s_idx - 1]
// - p[p_idx - 1]
//
// Base:
// dp[0][0] = true because empty string matches empty pattern.
//
// Empty String can also match patterns like `a*`, `a*b*`, etc.
// if p[p_idx - 1] is '*', then the `x*` group can be skipped:
// dp[0][p_idx] = dp[0][p_idx - 2]
//
// Transition:
//
// 1. If current pattern char is not `*`:
//    Current chars must match directly, or pattern char must be '.'.
//    Then the answer comes from the previous prefixes:
//
//    dp[s_idx][p_idx] = dp[s_idx - 1][p_idx - 1]
//
// 2. If current pattern char is '*':
//    `*` applies to previous pattern char, so the group is `x*`.
//
//    Option A: Use zero occurrences of `x*`:
//    dp[s_idx][p_idx - 2]
//
//    Option B: if `x` matches the current string char, use one more occurrence:
//    dp[s_idx - 1][p_idx]
//
//    We stay on the same pattern prefix because `x*` can match more chars.
//
// Time: O(s * p) each state is computed once
// Space: O(s * p) for dp

pub fn is_match_dp(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();

    let s_len = s.len();
    let p_len = p.len();

    let mut dp = vec![vec![false; p_len + 1]; s_len + 1];

    dp[0][0] = true;

    // Empty string against patterns a*b*..
    for p_idx in 1..=p_len {
        if p[p_idx - 1] == b'*' && p_idx >= 2 {
            dp[0][p_idx] = dp[0][p_idx - 2];
        }
    }

    for s_idx in 1..=s_len {
        for p_idx in 1..=p_len {
            // `*` applies to the previous pattern char, forming `x*`.
            if p[p_idx - 1] == b'*' {
                if p_idx < 2 {
                    continue;
                }

                // Use zero occurrences of `x*`
                dp[s_idx][p_idx] = dp[s_idx][p_idx - 2];

                // Use x* as one or more occurrences
                let prev = p[p_idx - 2];

                if prev == b'.' || prev == s[s_idx - 1] {
                    // Use one more occurrence of `x*` if `x` matches current s char.
                    // Keep p_idx the same because `x*` may match more chars.
                    dp[s_idx][p_idx] |= dp[s_idx - 1][p_idx];
                }
            } else {
                let is_match = p[p_idx - 1] == b'.' || p[p_idx - 1] == s[s_idx - 1];

                if is_match {
                    dp[s_idx][p_idx] = dp[s_idx - 1][p_idx - 1];
                }
            }
        }
    }

    dp[s_len][p_len]
}

// Top-down DP with DFS + memoization.
//
// dfs(s_idx, p_idx) = whether s[s_idx..] matches p[p_idx..].
//
// If the next pattern char is '*':
// - Skip the `char*` group: dfs(s_idx, p_idx + 2).
// - If current chars match, then consume one string char and stay on same pattern.
//
// Without '*', current chars must match, then both indices advance.
//
// Memo size includes + 1 for empty suffix states.
//
// Time: O(s * p), each state is computed once.
// Space: O(s * p) for memo plus recursion stack.
pub fn is_match_memo(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();

    let mut memo = vec![vec![None; p.len() + 1]; s.len() + 1];

    dfs(s, p, 0, 0, &mut memo)
}

fn dfs(s: &[u8], p: &[u8], s_idx: usize, p_idx: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
    if p_idx >= p.len() {
        // If both out of bounds then it's a match
        // If we still have chars in s then it's not match.
        // Code below takes care of s out of bounds while `p` still have chars.
        return s_idx >= s.len();
    }

    if let Some(res) = memo[s_idx][p_idx] {
        return res;
    }

    let is_match = s_idx < s.len() && (s[s_idx] == p[p_idx] || p[p_idx] == b'.');

    let res = if p_idx + 1 < p.len() && p[p_idx + 1] == b'*' {
        let skip_group = dfs(s, p, s_idx, p_idx + 2, memo);
        let use_one = is_match && dfs(s, p, s_idx + 1, p_idx, memo);

        skip_group || use_one
    } else if is_match {
        dfs(s, p, s_idx + 1, p_idx + 1, memo)
    } else {
        false
    };

    memo[s_idx][p_idx] = Some(res);
    res
}
