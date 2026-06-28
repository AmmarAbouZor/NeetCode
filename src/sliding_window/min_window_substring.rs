use std::collections::HashMap;

// Sliding Window with frequency counts;
//
// Expand `right` until the window contains all required characters with enough frequency (counts).
// Then move `left` while the window is still valid. updating the best window before removing from
// the left.
//
// `need` is the number of distinct required characters.
// `have` is the number of required characters whose current window count meets
//  the target count.
//
// Time: O(s.len() + t.len()) on average with HashMap
// Space: O(distinct chars in s and t) . Worst case O(s + t)

pub fn min_window(s: String, t: String) -> String {
    if s.is_empty() || t.is_empty() {
        return String::new();
    }

    // Bytes are simpler for indexing under the ASCII input constraint.
    let s = s.as_bytes();
    let t = t.as_bytes();

    let mut target = HashMap::new();
    for &ch in t {
        target.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut window = HashMap::new();

    let mut best_len = usize::MAX;
    let mut best_start = 0;

    let need = target.len();
    let mut have = 0;

    let mut left = 0;

    for (right, &ch) in s.iter().enumerate() {
        // Keep the updated count so we do not need another window lookup below.
        let ch_win_count: usize = {
            let count = window.entry(ch).or_default();
            *count += 1;
            *count
        };

        if target.get(&ch).is_some_and(|c| ch_win_count == *c) {
            have += 1;
        }

        while have == need {
            let win_len = right - left + 1;
            if win_len < best_len {
                best_len = win_len;
                best_start = left;
            }

            let left_ch = s[left];

            // Keep the updated count after removing from the left.
            let left_ch_win_count = {
                let count = window.get_mut(&left_ch).unwrap();
                *count -= 1;
                *count
            };

            if target.get(&left_ch).is_some_and(|c| left_ch_win_count < *c) {
                have -= 1;
            }

            left += 1;
        }
    }

    if best_len == usize::MAX {
        String::new()
    } else {
        String::from_utf8_lossy(&s[best_start..best_start + best_len]).into_owned()
    }
}
