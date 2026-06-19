// Expand around each possible center, checking both odd and even length palindromes.
// See longest_palindrome.rs for the same pattern returning the longest substring.

// Time: O(n^2)
// Space: O(1)

pub fn count_substrings(s: String) -> i32 {
    // Empty string case doesn't need special handling
    let s = s.as_bytes();
    let mut count = 0;

    for idx in 0..s.len() {
        // Odd case:
        increase_palindrome_count(idx, idx, s, &mut count);

        // Even case:
        increase_palindrome_count(idx, idx + 1, s, &mut count);
    }

    count as i32
}

fn increase_palindrome_count(mut l: usize, mut r: usize, s: &[u8], count: &mut usize) {
    while r < s.len() && s[l] == s[r] {
        *count += 1;
        l = match l.checked_sub(1) {
            Some(val) => val,
            None => break,
        };

        r += 1;
    }
}
