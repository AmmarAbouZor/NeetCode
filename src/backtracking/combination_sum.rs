//NOTE: Optimized Solution is below

// Combination DFS
// Time: O(2^(T/m)) as a loose bound for the search tree, where T is target
// and m is the smallest candidate. Cloning results adds output cost.
// Extra space: O(T/m) for recursion stack and current combination, excluding output.
// Output space: O(k * T/m), where k is the number of valid combinations.

// Combination with DFS to avoid dulicate permutations:
// - One branch choosing to keep including an item
// - Other branch to stop including the items and move to next item.

pub fn combination_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
    if target == total {
        res.push(cur.clone());
        return;
    }

    if idx >= nums.len() || total > target {
        return;
    }

    cur.push(nums[idx]);
    dfs(nums, target, idx, cur, total + nums[idx], res);

    cur.pop();
    dfs(nums, target, idx + 1, cur, total, res);
}

// Sorting lets us stop the loop once total + nums[j] exceeds target.
// Passing j into the recursive call allows reusing the same candidate while
// keeping combinations in non-decreasing index order, avoiding duplicate permutations.

// Time: O(2^(T/m)) as a loose upper bound, plus output cloning.
// Sorting adds O(n log n), usually dominated by the search.
// Extra space: O(T/m), excluding output.
// Output space: O(k * T/m).

pub fn combination_sum_optimized(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut res = Vec::new();

    dfs_opt(&nums, target, 0, &mut Vec::new(), 0, &mut res);

    res
}

pub fn dfs_opt(
    nums: &[i32],
    target: i32,
    idx: usize,
    cur: &mut Vec<i32>,
    total: i32,
    res: &mut Vec<Vec<i32>>,
) {
    if target == total {
        res.push(cur.clone());
        return;
    }

    for j in idx..nums.len() {
        if total + nums[j] > target {
            return;
        }

        cur.push(nums[j]);
        dfs_opt(nums, target, j, cur, total + nums[j], res);
        cur.pop();
    }
}
