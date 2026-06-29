// Top-down DP with DFS + memoization
//
// dp[r][c] = length of the longest increasing path start at cell (r, c).
//
// From each cell, try the 4 neighbors with strictly larger value. Memoization
// make each cell computed once.
//
// Time: O(rows * cols), Because each cell checks 4 neighbors once.
// Space: O(rows * cols) for dp, plus recursion stack.

pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut dp = vec![vec![0; cols]; rows];

    let mut best = 0;

    for r in 0..rows {
        for c in 0..cols {
            let dfs_res = dfs(&matrix, r, c, &mut dp);
            best = best.max(dfs_res);
        }
    }

    best
}

fn dfs(matrix: &[Vec<i32>], row: usize, col: usize, dp: &mut [Vec<i32>]) -> i32 {
    if dp[row][col] != 0 {
        return dp[row][col];
    }

    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut best = 1;

    for (dr, dc) in DIRS {
        let Some(next_r) = row.checked_add_signed(dr) else {
            continue;
        };
        let Some(next_c) = col.checked_add_signed(dc) else {
            continue;
        };

        if next_r >= matrix.len() || next_c >= matrix[0].len() {
            continue;
        }

        if matrix[next_r][next_c] > matrix[row][col] {
            let dfs_val = dfs(matrix, next_r, next_c, dp);
            best = best.max(1 + dfs_val);
        }
    }

    dp[row][col] = best;
    best
}
