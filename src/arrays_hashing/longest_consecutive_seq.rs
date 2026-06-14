//! We iterate once collecting items into the set then we can look up
//! if we have previous or next values in O(1). This will end up with
//! the solution with O(n)

// O(N). Using HashSets
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let nums_set: HashSet<i32> = nums.into_iter().collect();

    let mut total_max = 0;

    for &num in &nums_set {
        if nums_set.contains(&(num - 1)) {
            continue;
        }
        let mut len = 1;

        while nums_set.contains(&(num + len)) {
            len += 1;
        }

        total_max = total_max.max(len);
    }

    total_max
}
