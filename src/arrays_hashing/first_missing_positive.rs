// In-place index placement.
//
// For an array of length n, the first missing positive must be in 1..=n + 1.
// Place each value x in its target index x - 1 when 1 <= x <= n.
//
// After placement, if nums[i] != i + 1, then i + 1 is the first missing
// positive. If every index is correct, the answer is n + 1.
//
// The duplicate check is required before swapping. Without it, duplicate values
// like [1, 1] can cause an infinite loop.
//
// Time: O(n), because each swap puts at least one value into its correct place.
// Space: O(1), ignoring the input array.

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut nums = nums;

    for idx in 0..n {
        // Keep swapping current value into its correct index while it
        // is in range.
        while nums[idx] > 0 && nums[idx] as usize <= n {
            let target = nums[idx] as usize - 1;

            // Duplicate or already placed value; no useful swap remains.
            if nums[idx] == nums[target] {
                break;
            }

            nums.swap(idx, target);
        }
    }

    for (idx, &num) in nums.iter().enumerate() {
        if num as usize != idx + 1 {
            return idx as i32 + 1;
        }
    }

    n as i32 + 1
}
