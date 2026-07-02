use std::collections::BinaryHeap;

// Max-heap greedy.
//
// Always place the character with the largest remaining count, but keep the
// previously placed character out of the heap for one round. This prevent the
// same character from being placed adjacent to itself.
//
// `prev` stores the character that was just used and still have remaining count.
// After placing a difference character, `prev` can be pushed back into the heap.
//
// If the heap becomes empty while `prev` still exists, no valid reorganization
// is possible.
//
// Time: O(n log26), effectively O(n) for lowercase English characters.
// Space: O(n) including the output, O(1) extra space because the heap and count
// array are bounded by 26.

pub fn reorganize_string(s: String) -> String {
    let mut char_count = [0; 26];

    for &ch in s.as_bytes() {
        let idx = (ch - b'a') as usize;
        char_count[idx] += 1;
    }

    let mut heap: BinaryHeap<(usize, u8)> = char_count
        .into_iter()
        .enumerate()
        .filter(|(_idx, c)| *c > 0)
        .map(|(idx, c)| (c, (idx as u8 + b'a')))
        .collect();

    let mut res = Vec::with_capacity(s.len());
    let mut prev: Option<(usize, u8)> = None;

    while let Some((mut count, ch)) = heap.pop() {
        res.push(ch);
        count -= 1;

        if let Some((pr_count, pr_ch)) = prev.take() {
            heap.push((pr_count, pr_ch));
        }

        if count > 0 {
            prev = Some((count, ch));
        }
    }

    if prev.is_some() {
        String::new()
    } else {
        String::from_utf8(res).expect("Input is lower case chars")
    }
}
