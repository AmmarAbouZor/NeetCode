// Backtracking keeping track on indexes of elements which are used at that state.
// The branch/undo will be to add the value to the current permutation and mark it as used,
// Once the branch is done then we pop it back and mark it as unused.

// It's possible also to do that with swapping the items in the input but it will be
// hard to explain and reason about (even the code is shorter).

// Time: O(n * n!), because there are n! permutations and cloning each one costs O(n).
// Extra space: O(n) for recursion, used, and current permutation
// Output space: O(n * n!)

// Note: The brute-force with recursion solution will end up with O(n! * n^2).

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut used = vec![false; nums.len()];

    dfs(&nums, &mut used, &mut Vec::new(), &mut res);

    res
}

fn dfs(nums: &[i32], used: &mut [bool], curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if curr.len() == nums.len() {
        res.push(curr.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        curr.push(nums[i]);
        used[i] = true;

        dfs(nums, used, curr, res);

        curr.pop();
        used[i] = false;
    }
}
