// Full-table DP with explicit first row and first column initialization.
// Check the optimized solution below
//
// dp[r][c] = minimum path sum to reach cell (r, c).
// The first row can only come from the left, and the first column can only
// come from above. Inner cells choose the cheaper of top and left.
//
// Time: O(m * n)
// Space: O(m * n)
pub fn min_path_sum_better_base(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();

    let inf = i32::MAX / 2;
    let mut dp = vec![vec![inf; n]; m];
    dp[0][0] = grid[0][0];

    for r in 1..m {
        dp[r][0] = dp[r - 1][0] + grid[r][0];
    }

    for c in 1..n {
        dp[0][c] = dp[0][c - 1] + grid[0][c];
    }

    for r in 1..m {
        for c in 1..n {
            dp[r][c] = grid[r][c] + dp[r - 1][c].min(dp[r][c - 1]);
        }
    }

    dp[m - 1][n - 1]
}

// Space-optimized grid DP.
//
// dp[c] is the minimum path sum to reach column c in the current row.
// Before updating, dp[c] is the value from top. After updating, dp[c - 1]
// is the value from left in the current row.
//
// Time: O(m * n)
// Space: O(n)
pub fn min_path_sum_optimized(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let cols = grid[0].len();

    let inf = i32::MAX / 2;
    let mut dp = vec![inf; cols];
    dp[0] = 0;

    for row in &grid {
        for c in 0..cols {
            if c == 0 {
                dp[c] += row[c];
            } else {
                dp[c] = row[c] + dp[c].min(dp[c - 1]);
            }
        }
    }

    dp[cols - 1]
}

// Full-table DP with a uniform loop and sentinel values.
//
// Initialize cells to infinity so boundary cells can use the same top/left
// update rules without pre-filling the first row and column.
//
// Time: O(m * n)
// Space: O(m * n)
pub fn min_path_sum_basic_base(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();

    let inf = i32::MAX / 2;
    let mut dp = vec![vec![inf; n]; m];

    dp[0][0] = grid[0][0];

    for r in 0..m {
        for c in 0..n {
            let val = grid[r][c];
            if r > 0 {
                dp[r][c] = val + dp[r - 1][c];
            }

            if c > 0 {
                dp[r][c] = dp[r][c].min(val + dp[r][c - 1]);
            }
        }
    }

    dp[m - 1][n - 1]
}
