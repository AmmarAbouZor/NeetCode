// Backtracking DFS,
// Time: O(n * 2^n)
// Extra temporary space: O(n)
// Output space: O(n * 2^n

pub fn subsets_dfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut subset = Vec::new();
    dfs(&nums, 0, &mut subset, &mut res);

    res
}

fn dfs(nums: &[i32], idx: usize, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if idx >= nums.len() {
        res.push(subset.clone());
        return;
    }

    subset.push(nums[idx]);
    dfs(nums, idx + 1, subset, res);

    subset.pop();
    dfs(nums, idx + 1, subset, res);
}

// Resolve dfs with loop:
// At every DFS call, the current subset is already valid, so save it immediately.
// Then try adding each later element.
#[allow(unused)]
fn dfs_loop(nums: &[i32], idx: usize, subset: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    res.push(subset.clone());

    for j in idx..nums.len() {
        subset.push(nums[j]);
        dfs_loop(nums, j + 1, subset, res);
        subset.pop();
    }
}

// Iterative subset expansion,
// Time: O(n * 2^n)
// Extra space: O(n), for recursion stack and current subset.
// Output space: O(n * 2^n), because there are 2^n subsets and each can contain up to n elements.

pub fn subsets_iter(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![Vec::new()];

    for num in nums {
        let size = res.len();
        for idx in 0..size {
            let mut new_subset = res[idx].clone();
            new_subset.push(num);
            res.push(new_subset);
        }
    }

    res
}
