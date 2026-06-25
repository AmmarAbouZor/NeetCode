// Greedy idea:
// * Record the last position of every character.
// * Start a partition and keep extending its end to the farthest last position
//   of any character seen inside it.
// * When the current index passes the end, the partition is complete.

// Time: O(n) iterate through s twice
// Space: O(26) = O(1)

// This solution is more compact but the one below is easier to understand.
pub fn partition_labels_for_loop(s: String) -> Vec<i32> {
    let s = s.as_bytes();
    let mut last_pos = [0; 26];

    for (idx, &b) in s.iter().enumerate() {
        let b_idx = b - b'a';
        last_pos[b_idx as usize] = idx;
    }

    let mut res = Vec::new();
    let mut start = 0;
    let mut end = 0;

    for (idx, &b) in s.iter().enumerate() {
        let b_idx = b - b'a';
        end = end.max(last_pos[b_idx as usize]);

        if idx == end {
            let len = end - start + 1;
            res.push(len as i32);
            start = idx + 1;
        }
    }

    res
}

// Slightly longer solution with while loop but easier to follow.
pub fn partition_labels(s: String) -> Vec<i32> {
    if s.is_empty() {
        return Vec::new();
    }
    let s = s.as_bytes();
    let mut last_pos = [0; 26];

    for (idx, &b) in s.iter().enumerate() {
        let b_idx = b - b'a';
        last_pos[b_idx as usize] = idx;
    }

    let mut res = Vec::new();

    let mut cur = 0;

    while cur < s.len() {
        let start = cur;
        let mut end = cur;
        while cur <= end {
            let ch = s[cur];
            let b_idx = ch - b'a';
            end = end.max(last_pos[b_idx as usize]);
            cur += 1;
        }
        let len = cur - start;
        res.push(len as i32);
    }

    res
}
