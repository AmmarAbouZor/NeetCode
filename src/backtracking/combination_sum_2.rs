// Combinations DFS with sorted input
// Time: O(n * 2^n), because each candidate can be included or skipped,
// and cloning each valid combination can cost up to O(n).
// Extra space: O(n), for recursion stack and current combination, excluding output.
// Output space: O(k * n), where k is the number of valid combinations.

// Main pattern:
// sort
// dfs(start, current, total/remaining)
// for i in start..nums.len() {
//     skip duplicates at same depth
//     prune if nums[i] > remaining
//     choose nums[i]
//     dfs(next_start, ...)
//     undo
// }

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = candidates;
    nums.sort_unstable();

    let mut res = Vec::new();

    dfs(&nums, target, 0, &mut Vec::new(), 0, &mut res);

    res
}

fn dfs(
    nums: &[i32],
    target: i32,
    idx: usize,
    cur: &mut Vec<i32>,
    total: i32,
    res: &mut Vec<Vec<i32>>,
) {
    if total == target {
        res.push(cur.clone());
        return;
    }

    for j in idx..nums.len() {
        // Skip duplicate sibling choices at this DFS level.
        // Duplicates can still be used across deeper levels when they come from
        // different indices, but this prevents generating the same combination twice.
        if j > idx && nums[j] == nums[j - 1] {
            continue;
        }

        // No need to continue if sum has exceeded target as nums is sorted.
        if total + nums[j] > target {
            return;
        }

        cur.push(nums[j]);
        dfs(nums, target, j + 1, cur, total + nums[j], res);
        let _ = cur.pop();
    }
}
