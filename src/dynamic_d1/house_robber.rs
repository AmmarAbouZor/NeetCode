// Dynamic Programming with two variables instead of full dp array.
// Refer to the explanation for dp function below.
// Note: Also check the version with iterator as more idiomatic rust.

// Time O(n)
// Space O(1)
pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    match n {
        0 => return 0,
        1 => return nums[0],
        _ => {}
    }

    let mut prev1 = nums[0].max(nums[1]); // dp[i - 1]
    let mut prev2 = nums[0]; // dp[i - 2]

    for num in nums.into_iter().skip(2) {
        let cur = prev1.max(num + prev2);
        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}

// Solution with dp array:
// At each house, choose between skipping it and keeping dp[i - 1],
// or robbing it and adding nums[i] to dp[i - 2].
// We start by manually implementing that for indexes 0 and 1 then we put that into a loop.
// We don't need to calculate to n + 1 here as we stop at the last house.

// Time: O(n)
// Space: O(n)

pub fn rob_dp(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    match n {
        0 => return 0,
        1 => return nums[0],
        _ => {}
    }

    let mut dp = vec![0; n];
    dp[0] = nums[0];
    dp[1] = nums[0].max(nums[1]);

    for i in 2..n {
        dp[i] = dp[i - 1].max(nums[i] + dp[i - 2]);
    }

    dp[n - 1]
}

// Solution with two variables but using iterator to skip indexing and
// manual bound checks.
// This will more confusing as helpful in interview.
pub fn rob_iter(nums: Vec<i32>) -> i32 {
    let mut nums = nums.into_iter();
    let Some(first_house) = nums.next() else {
        return 0;
    };
    let Some(second_house) = nums.next() else {
        return first_house;
    };
    let mut prev1 = first_house.max(second_house);
    let mut prev2 = first_house;

    for num in nums {
        let cur = prev1.max(num + prev2);
        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}
