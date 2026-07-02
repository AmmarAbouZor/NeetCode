use std::collections::BinaryHeap;

// Max-heap greedy.
//
// Always try to use the character with the largest remaining count. If adding
// it would create three equal characters in a row, use the next most frequent
// character instead.
//
// If the most frequent character is blocked and there is no second choice,
// we cannot extend the string anymore.
//
// Time: O(n log 3), effectively O(n), where n = a + b + c.
// Space: O(n) including output, O(1) extra space because the heap has at
// most 3 characters.
pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut heap: BinaryHeap<(i32, u8)> = [(a, b'a'), (b, b'b'), (c, b'c')]
        .into_iter()
        .filter(|(count, _ch)| *count > 0)
        .collect();

    let mut res = Vec::with_capacity((a + b + c) as usize);

    while let Some((mut count, ch)) = heap.pop() {
        let len = res.len();
        // The best character is temporarily blocked because it would
        // make `xxx`.
        if len >= 2 && res[len - 1] == ch && res[len - 2] == ch {
            // Use the next best char to break the run.
            let Some((mut next_count, next_ch)) = heap.pop() else {
                break;
            };

            res.push(next_ch);
            next_count -= 1;

            if next_count > 0 {
                heap.push((next_count, next_ch));
            }

            heap.push((count, ch));
        } else {
            res.push(ch);
            count -= 1;

            if count > 0 {
                heap.push((count, ch));
            }
        }
    }

    String::from_utf8(res).expect("Only a, b, c chars are used")
}
