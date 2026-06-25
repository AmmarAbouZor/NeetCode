// Dynamic programming over prefixes.
//
// dp[i] = true if s[0..i] can be segmented into words from the dictionary.
//
// For each end index i, try every split point j:
// - dp[j] means the prefix before j is already valid
// - s[j..i] must be a word in the dictionary
//
// If both are true, then dp[i] is true.
//
// Let n = s.len() and k = total characters in word_dict.
// Time: O(k + n^3) worst case, because there are O(n^2) splits and each
// substring lookup hashes up to O(n) chars.
// Space: O(k + n), for the HashSet and dp.

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let words: std::collections::HashSet<String> = word_dict.into_iter().collect();

    let n = s.len();

    let mut dp = vec![false; n + 1];

    dp[0] = true;

    for idx in 1..=n {
        for j in 0..idx {
            if dp[j] && words.contains(&s[j..idx]) {
                dp[idx] = true;
                break;
            }
        }
    }

    dp[n]
}
