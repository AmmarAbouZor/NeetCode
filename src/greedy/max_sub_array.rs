// Using Kadane's Algorithm, based on:
// * if the running sum becomes negative, keeping it will only reduce any future subarray sum.

// The sum value of each node will be the maximum between including previous sum with the current value or not.
// We don't need to keep track on all node values (The DP solution). We need to keep track on the maximum value
// among them.

// Time: O(n) one linear iteration
// Space: O(1)

// Note: The brute force solution will start with O(n^3) and it can be reduced to O(n^2)

// cur_sum is the best subarray sum ending at the current index:
// either start fresh at nums[i], or extend the previous subarray.
pub fn max_sub_array_kadane_iter(nums: Vec<i32>) -> i32 {
    let mut nums = nums.into_iter();
    let Some(mut cur_sum) = nums.next() else {
        return 0;
    };
    let mut max_sum = cur_sum;

    for num in nums {
        cur_sum = num.max(cur_sum + num);
        max_sum = max_sum.max(cur_sum);
    }

    max_sum
}

pub fn max_sub_array_kadane(nums: Vec<i32>) -> i32 {
    let Some(&first) = nums.first() else {
        return 0;
    };

    let mut cur_sum = first;
    let mut max_sum = first;

    for &num in nums.iter().skip(1) {
        cur_sum = num.max(cur_sum + num);
        max_sum = max_sum.max(cur_sum);
    }

    max_sum
}

// Time: O(n)
// Space: O(n)
pub fn max_sub_array_dp(nums: Vec<i32>) -> i32 {
    let mut dp = nums.clone();
    for i in 1..nums.len() {
        dp[i] = nums[i].max(dp[i - 1] + nums[i]);
    }

    dp.into_iter().max().unwrap_or_default()
}
