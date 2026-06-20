// Solution based on n ^ n = 0 and n ^ 0 = n
// So we can just xor all numbers together and get the result

// Time: O(n) iteration over items
// Space: O(1)

pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, n| acc ^ n)
}

// This is the standard implementation using xor operation
pub fn single_number_standard(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for num in nums {
        res ^= num;
    }

    res
}
