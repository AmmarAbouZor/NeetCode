// Greedy algorithm based on starting from the goal point and move it
// to the previous points if they can jump to the goal.
// The idea that we don't need to check for the final target if we can check
// a checkpoint where we know that can reach the final target.

// Greedy invariant:
// goal is the leftmost index known to be able to reach the end. If index i can reach goal,
// then i becomes the new goal.

// Time: O(n) for one linear iteration.
// Space: O(1)

// Note: Brute force is a bizarre O(m ^ n): where m is maximum number of jumps.

pub fn can_jump(nums: Vec<i32>) -> bool {
    match nums.len() {
        0 => return false,
        1 => return true,
        _ => {}
    }

    let mut goal = nums.len() - 1;
    for i in (0..nums.len() - 1).rev() {
        let distance = goal - i;
        if nums[i] >= distance as i32 {
            goal = i;
        }
    }

    goal == 0
}
