// O(logn) time binary search. and O(1) space
// The point here that I need to compare with the right bound and
// not keep track on the last middle.
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    nums[left]
}
