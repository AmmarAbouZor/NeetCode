// Avoid dealing with usize bounds by using upper bound making:
// * r = nums.len()
// * while l < r {
// * Ordering::Greater => r = mid
//
// This will avoid many problems for checking for usize bounds and empty input.

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let mid = l + (r - l) / 2;

        use std::cmp::Ordering;
        match nums[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => l = mid + 1,
            Ordering::Greater => r = mid,
        }
    }

    -1
}

pub fn search_my_signature(nums: &[i32], target: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let mid = l + (r - l) / 2;
        use std::cmp::Ordering;

        match nums[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => l = mid + 1,
            Ordering::Greater => r = mid,
        }
    }

    None
}
