// Check the optimized solution with HashMap.

// Time O(N): Two pointers algorithm
// Space O(M): Where M is the longest chars sequence (Can be O(N) in worst case).
pub fn length_of_longest_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut char_set = std::collections::HashSet::new();
    let mut left = 0;
    let mut longest = 0;

    for right in 0..bytes.len() {
        while char_set.contains(&bytes[right]) {
            char_set.remove(&bytes[left]);
            left += 1;
        }
        char_set.insert(bytes[right]);
        longest = longest.max(right - left + 1);
    }

    longest as i32
}

// Also O(N) and O(M) but uses HashMap to avoid repeatedly remove left chars
// And advance it directly to the prev indexed of right.
pub fn length_of_longest_substring_hashmap(s: String) -> i32 {
    let mut last_seen = std::collections::HashMap::new();
    let mut left = 0;
    let mut longest = 0;

    for (right, byte) in s.bytes().enumerate() {
        if let Some(prev_idx) = last_seen.insert(byte, right)
            && prev_idx >= left
        {
            left = prev_idx + 1;
        }

        longest = longest.max(right - left + 1);
    }

    longest as i32
}

// This is O(n^2) as we are going back
pub fn length_of_longest_substring_not_good(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let bytes = s.as_bytes();

    pub use std::collections::HashSet;
    let mut chars_set = HashSet::new();

    let mut left = 0;
    let mut right = 1;

    chars_set.insert(bytes[left]);

    let mut longest = 1;
    while right < s.len() {
        if chars_set.insert(bytes[right]) {
            longest = chars_set.len().max(longest);
            right += 1;
        } else {
            chars_set.clear();
            left += 1;
            chars_set.insert(bytes[left]);
            right = left + 1;
        }
    }

    longest as i32
}
