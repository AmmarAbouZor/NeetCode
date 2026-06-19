// Space-optimized DP.
//
// At each character, there are at most two valid choices:
// - decode the current digit alone, if it is '1'..='9'
// - decode the previous and current digits together, if they form 10..=26
//
// prev1 is the number of ways to decode up to the previous character.
// prev2 is the number of ways to decode up to the character before that.
//
// Time: O(n)
// Space: O(1)
pub fn num_decodings_var(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let s = s.as_bytes();
    let n = s.len();

    // Empty prefix contributes one path for valid two-digit starts like "10" or "12".
    let mut prev2 = 1;
    let mut prev1 = if s[0] == b'0' { 0 } else { 1 };

    for idx in 1..n {
        let mut cur = 0;

        // Use current digit alone. '0' cannot be decoded by itself.
        if s[idx] != b'0' {
            cur += prev1;
        }

        // Use previous + current as one letter.
        let two_digits_num = (s[idx - 1] - b'0') * 10 + (s[idx] - b'0');
        if (10..=26).contains(&two_digits_num) {
            cur += prev2;
        }

        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}

// Bottom-up DP using a full array.
//
// dp[i] is the number of ways to decode the first i characters.
// This uses prefix length instead of string index, so dp has n + 1 entries.
//
// For each prefix ending at i:
// - if s[i - 1] is not '0', append it as a one-digit decode to each dp[i - 1] way
// - if s[i - 2..i] is 10..=26, append it as a two-digit decode to each dp[i - 2] way
//
// Time: O(n)
// Space: O(n)
pub fn num_decodings_dp(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let s = s.as_bytes();
    let n = s.len();

    let mut dp = vec![0; n + 1];
    // Empty prefix has one decoding path. This lets valid two-digit prefixes
    // like "10" and "12" contribute dp[0].
    dp[0] = 1;
    dp[1] = if s[0] == b'0' { 0 } else { 1 };

    for idx in 2..=n {
        if s[idx - 1] != b'0' {
            dp[idx] += dp[idx - 1];
        }

        let two_digits_num = (s[idx - 2] - b'0') * 10 + (s[idx - 1] - b'0');
        if (10..=26).contains(&two_digits_num) {
            dp[idx] += dp[idx - 2];
        }
    }

    dp[n]
}
