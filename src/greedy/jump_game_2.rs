// The greedy solution here feels like a sliding window:
// - We decide the furthest that we can reach on each jump as the end of next window.
// - In each window we calculate the furthest that we can reach from it.
// - We increase the counter on each jump and stop once reached the target.

// Time: O(n) linear iteration
// Space: O(1)

// Note:
// This solution assumes that the input is valid and always solvable.
// The second function below handles unreachable input without running Jump Game I first.

// Using enumerate with skip()/take() instead of idx loop will make it too complex for the interview.
#[allow(clippy::needless_range_loop)]
pub fn jump(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut l = 0;
    let mut r = 0;

    while r < nums.len() - 1 {
        let mut furthest = 0;
        for idx in l..=r {
            furthest = furthest.max(nums[idx] as usize + idx);
        }
        l = r + 1;
        r = furthest;
        count += 1;
    }

    count
}

// Solution with checks for invalid and none-resolvable input.
#[allow(clippy::needless_range_loop)]
pub fn jump_with_validation(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => return -1,
        1 => return 0,
        _ => {}
    }

    let mut count = 0;
    let mut l = 0;
    let mut r = 0;

    while r < nums.len() - 1 {
        let mut furthest = 0;
        for idx in l..=r {
            furthest = furthest.max(nums[idx] as usize + idx);
        }
        // We would get stuck because the next window cannot move past r.
        if furthest == r {
            return -1;
        }
        l = r + 1;
        r = furthest;
        count += 1;
    }

    count
}
