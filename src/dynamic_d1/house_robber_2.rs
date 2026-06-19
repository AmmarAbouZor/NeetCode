// Difference between house robber 1 and 2:
// We need to account for the case that we can't rob the first and last
// houses at the same time.
// The most simple way is to run house robber 1 function once with skipping the first house
// and once with skipping the last house then to pick the bigger of them.

// Time: O(2 * n) = O(n)
// Space: O(1)

pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    match n {
        0 => return 0,
        1 => return nums[0],
        _ => {}
    }

    let skip_last = house_robber_1(&nums[..nums.len() - 1]);
    let skip_first = house_robber_1(&nums[1..]);

    skip_last.max(skip_first)
}

// The is the most simple version of house robber 1.
// For more info refer to house robber 1.
fn house_robber_1(nums: &[i32]) -> i32 {
    let mut prev1 = 0;
    let mut prev2 = 0;
    for num in nums {
        let cur = prev1.max(num + prev2);
        prev2 = prev1;
        prev1 = cur;
    }

    prev1
}
