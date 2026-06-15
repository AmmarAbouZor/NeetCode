// O(logN) for binary search. O(1) Space

// Solution is binary search but it needs time to think about the conditions in which
// The array is rotated.

// Current solution is different than one in NeetCode as it's easier to reason about.
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0_i32;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        let mid_num = nums[mid as usize];

        if mid_num == target {
            return mid;
        }

        let left_num = nums[left as usize];

        if left_num <= mid_num {
            if left_num <= target && target < mid_num {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            let right_num = nums[right as usize];
            if mid_num < target && target <= right_num {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    -1
}
