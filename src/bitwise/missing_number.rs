// The trick here is to find the xor for the range 0..n
// and xor it to the current numbers making existing numbers
// Clear each others out keeping the missing number

// Time: O(n) for iteration.
// Space O(1)

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    // Equivalent to
    //  let mut res = 0;
    //  res ^= n;
    let mut res = n;
    for (idx, num) in nums.into_iter().enumerate() {
        res ^= idx;
        res ^= num as usize;
    }

    res as i32
}
