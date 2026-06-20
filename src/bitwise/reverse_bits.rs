// We need to isolate each bit (using right shift + 1) and
// Adding it to the result using left shift for each bit.

// Time: O(32) ~= O(1)
// Space O(1)

pub fn reverse_bits(n: u32) -> u32 {
    let mut res = 0;
    for i in 0..32 {
        let bit = (n >> i) & 1;
        res |= bit << (31 - i)
    }

    res
}
