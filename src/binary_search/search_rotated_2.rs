// Binary search in a rotated sorted array with duplicates.
//
// Same idea as Search in Rotated Sorted Array I:
// at each step, try to identify which half is sorted, then decide whether the
// target lies inside that sorted half.
//
// The extra duplicate case is when nums[left] == nums[mid]. Then we cannot tell
// which half is sorted, so we safely move `left` by one to make progress.
//
// Because duplicates can force shrinking one step at a time, worst-case time is
// O(n), though typical cases still behave like O(log n).
//
// Time: O(log n) average, O(n) worst case with many duplicates.
// Space: O(1)

pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut left = 0_i32;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        let mid_val = nums[mid as usize];
        if mid_val == target {
            return true;
        }

        let left_val = nums[left as usize];
        let right_val = nums[right as usize];

        if left_val < mid_val {
            if target >= left_val && target < mid_val {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else if left_val > mid_val {
            if target <= right_val && target > mid_val {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        } else {
            // Duplicates hide the sorted half, so shrink safely.
            left += 1;
        }
    }

    false
}
