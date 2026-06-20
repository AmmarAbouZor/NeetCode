// Same as count bits for one number but apply it to all number from 0 to n
// Solutions this time are using iterator operations.

// Time: O(32 * n) ~= O(n) for iteration
// Space: O(n) for output

pub fn count_bits(n: i32) -> Vec<i32> {
    let n = n as u32;
    (0..=n).map(count_one).collect()
}

fn count_one(n: u32) -> i32 {
    (0..32).filter(|i| (1 << i) & n != 0).count() as i32
}
