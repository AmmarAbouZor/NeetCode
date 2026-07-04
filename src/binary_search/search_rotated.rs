// Binary search in a rotated sorted array with distinct values.
//
// At each step, one side of the array must be sorted:
// - if nums[left] <= nums[mid], the left half is sorted
// - otherwise, the right half is sorted
//
// Once we know the sorted half, check whether the target lies inside it.
// If it does, search that half; otherwise, search the other half.
//
// This is the version without duplicates. With duplicates, nums[left] == nums[mid]
// can hide which half is sorted, so Search in Rotated Sorted Array II needs an
// extra shrink step.
//
// Time: O(log n)
// Space: O(1)
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
