// Using floyd algorithm to detect linked list cycling.
// If we consider the values of nums as pointers (or indexes) to the same array
// then the duplicate value will be the cycle.
// This apply only when we guarantee that the start of the loop isn't part of the cycle, which
// is guaranteed here as values are [1, n] so 0 isn't included.
//
// XOR doesn't work here as the duplicated value isn't guaranteed to be duplicated once only.

// Time: O(n)
// Space: O(1) without changing nums's values.

pub fn find_duplicate_floyd(nums: Vec<i32>) -> i32 {
    // First stage: Interception between slow & fast pointers
    let mut slow = 0;
    let mut fast = 0;
    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        if slow == fast {
            break;
        }
    }

    // Second stage: Interception between two slow pointers:
    // * One starts from the interception with fast pointer from stage 1
    // * Second starts from the start of the list.
    let mut slow2 = 0;
    loop {
        slow = nums[slow] as usize;
        slow2 = nums[slow2] as usize;
        if slow == slow2 {
            return slow as i32;
        }
    }
}

// Time: O(N), we iterate through the nums.
// Space: O(1), we use nums itself but we are changing the array.
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    for i in 0..nums.len() {
        let num = nums[i].abs();
        let n_idx = num as usize;
        if nums[n_idx] < 0 {
            return num;
        }

        nums[n_idx] *= -1;
    }

    -1
}
