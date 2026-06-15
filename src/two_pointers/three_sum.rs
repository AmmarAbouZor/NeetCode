// O(n^2): time
// * sorting is O(NLogN)
// * two_sums is O(N) called from with loop in nums => O(N^2)

// Extra space: O(1), excluding the returned result
// Output space: O(k), where k is number of triplets, worst-case O(n^2)

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut res = Vec::new();

    for idx in 0..nums.len() {
        // Check for duplicates. This manual check to avoid using HashSet
        if idx > 0 && nums[idx] == nums[idx - 1] {
            continue;
        }

        let mut left = idx + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[idx] + nums[left] + nums[right];
            use std::cmp::Ordering;

            match sum.cmp(&0) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => {
                    res.push(vec![nums[idx], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    // Check for duplication manually as nums is sorted.
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }
    }

    res
}

// This solution allocate space for HashSet for duplicates and also 
// nesting two_sums making it allocate vec for its results as well.
pub fn three_sum_with_set(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;

    nums.sort_unstable();
    let mut ans = std::collections::HashSet::new();
    for (idx, &num) in nums.iter().enumerate() {
        let complement = 0 - num;
        for (l, r) in two_sums(&nums[idx + 1..], complement) {
            ans.insert(vec![num, l, r]);
        }
    }

    ans.into_iter().collect()
}

fn two_sums(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    if nums.len() < 2 {
        return Vec::new();
    }

    let mut l_idx = 0;
    let mut r_idx = nums.len() - 1;

    let mut ans = Vec::new();
    while l_idx < r_idx {
        let left = nums[l_idx];
        let right = nums[r_idx];

        let sum = left + right;

        use std::cmp::Ordering;
        match sum.cmp(&target) {
            Ordering::Equal => {
                ans.push((left, right));
                l_idx += 1;
            }
            Ordering::Less => l_idx += 1,
            Ordering::Greater => r_idx -= 1,
        }
    }

    ans
}
