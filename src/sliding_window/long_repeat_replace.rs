// Solution based on finding max count of any char in the window then check if
// `window_len - max_char_count <= k` as window validation
// If valid move with right, otherwise keep shifting left until it become valid again.
//
// Time O(N): Sliding window + O(26) in is_valid which is ignored as static
// Space O(1) which is actually O(26).

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut char_counts = [0_i32; 26];

    let mut left = 0;

    let mut longest = 0;
    let s = s.as_bytes();

    for right in 0..s.len() {
        char_counts[(s[right] - b'A') as usize] += 1;

        while !is_valid(left, right, &char_counts, k) {
            char_counts[(s[left] - b'A') as usize] -= 1;
            left += 1;
        }

        longest = longest.max(right - left + 1);
    }

    longest as i32
}

fn is_valid(left: usize, right: usize, char_counts: &[i32], k: i32) -> bool {
    let seq_len = right - left + 1;
    let max_char = *char_counts.iter().max().unwrap();

    seq_len as i32 - max_char <= k
}
