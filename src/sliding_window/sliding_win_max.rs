use std::collections::VecDeque;

// Monotonic decreasing deque of indices.
//
// The front of the deque is always the index of the largest value in the current window.
// Before pushing `right`, we remove indices from the back while their values are smaller
// than the current nums[right], because they can never be the max of this or any future window
// that includes nums[right].
//
// Once the first full window is formed, we remove indices from the front if they are outside
// of the current window.
//
// Time: O(n), because each index is pushed and popped at most once.
// Space: O(k) for the deque.

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;

    if nums.is_empty() || k == 0 {
        return Vec::new();
    }

    let mut res = Vec::new();

    let mut queue = VecDeque::new();

    for (right, &r_num) in nums.iter().enumerate() {
        // Remove smaller values from the back; `right` dominates them.
        while let Some(&idx) = queue.back() {
            if nums[idx] < r_num {
                queue.pop_back();
            } else {
                break;
            }
        }

        queue.push_back(right);

        if right + 1 >= k {
            let left = right + 1 - k;

            // Discard indices that are no longer inside this window.
            while queue.front().is_some_and(|i| *i < left) {
                queue.pop_front();
            }

            // The front is the max for the current window.
            let max_idx = *queue.front().unwrap();
            res.push(nums[max_idx]);
        }
    }

    res
}
