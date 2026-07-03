// Binary search on the answer.
//
// We want to minimize the largest subarray sum after splitting nums into k
// non-empty continuous parts.
//
// The answer must be between:
// - max(nums): no part can be smaller than the largest single number
// - sum(nums): putting everything into one part
//
// For a candidate `limit`, greedily count how many parts are needed if each
// part's sum must be <= limit. If we need at most k parts, the limit is valid
// and we try smaller. Otherwise, the limit is too small.
//
// Time: O(n * log(sum(nums) - max(nums)))
// Space: O(1)

pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    let Some(mut left) = nums.iter().max().copied() else {
        // nums is empty case.
        return 0;
    };
    let mut right: i32 = nums.iter().sum();

    while left < right {
        let mid = left + (right - left) / 2;

        if can_split(&nums, k as usize, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

// Return true if nums can be split into at most k parts where each part has
// sum <= limit.
//
// Greedy is valid because all numbers are non-negative: keep adding to the
// current part until adding the next number would exceed the limit, then start
// a new part.
fn can_split(nums: &[i32], k: usize, limit: i32) -> bool {
    let mut parts = 1;
    let mut sum = 0;

    for &num in nums {
        if sum + num > limit {
            parts += 1;
            sum = num;
        } else {
            sum += num;
        }
    }

    parts <= k
}
