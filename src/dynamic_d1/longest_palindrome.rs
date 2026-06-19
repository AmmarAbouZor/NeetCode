// Short explanation:
// This is solved without DP. DP is also O(n^2), but expand-around-center
// gives O(1) extra space.
// For each index, expand for both odd and even length palindromes.

// My equivalent rambling:
// This is solved without dp as it will end up with O(n^2) like the solution with two pointers.
// Solution here is to use two pointer approach but from the start and expand
// toward the edge keeping track on it. We will need to check for each each index for
// the odd and even cases.

// Time: O(n^2), because there are O(n) centers and each expansion can take O(n).
// Space: O(1), excluding the returned string.

pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::default();
    }

    let s = s.as_bytes();
    let mut max_len = 1;
    let mut max_start = 0;

    for idx in 0..s.len() {
        // Odd case:
        update_max(idx, idx, s, &mut max_len, &mut max_start);

        // Even case
        update_max(idx, idx + 1, s, &mut max_len, &mut max_start);
    }

    String::from_utf8_lossy(&s[max_start..max_start + max_len]).into_owned()
}

fn update_max(mut l: usize, mut r: usize, s: &[u8], max_len: &mut usize, max_start: &mut usize) {
    while r < s.len() && s[l] == s[r] {
        let diff = r - l + 1;
        if diff > *max_len {
            *max_len = diff;
            *max_start = l;
        }
        l = match l.checked_sub(1) {
            Some(val) => val,
            None => break,
        };
        r += 1;
    }
}
