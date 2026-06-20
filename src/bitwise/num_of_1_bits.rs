// Bit masking with `left shift` and `and` operations

// Time: O(32) = O(1)
// Space: O(1)

pub fn hamming_weight(n: u32) -> i32 {
    let mut res = 0;
    for i in 0..32 {
        if (1 << i) & n != 0 {
            res += 1;
        }
    }

    res
}
