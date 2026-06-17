// Backtracking DFS. Input contains duplicates so we sort it.
// Rest follow the same pattern.

// Time: O(2^n * n): 2^n for subsets, n for clone() call each one of them.
// Space without answer: O(n) recursion stack
// Space with answer: O(2^n * n)

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut res = Vec::new();

    dfs(&nums, 0, &mut Vec::new(), &mut res);

    res
}

fn dfs(nums: &[i32], idx: usize, curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    // Every subset in dfs calls is valid as we don't have length constrains
    res.push(curr.clone());

    for j in idx..nums.len() {
        if j > idx && nums[j] == nums[j - 1] {
            continue;
        }

        curr.push(nums[j]);
        dfs(nums, j + 1, curr, res);
        curr.pop();
    }
}
