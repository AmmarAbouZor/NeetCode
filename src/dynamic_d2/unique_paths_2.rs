// Full-table DP. Please check the optimized version.
//
// dp[rc][c] = number of ways to reach cell (r, c).
//
// if obstacle_grid[r][c] == 1, leave dp[r][c] as 0.
// Otherwise add ways from top and left.
//
// Unlike Unique Paths I, the first row/col are not automatically 1 because
// and obstacle can block the path.
//
// Time: O(m * n)
// Space: O(m * n)

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    if obstacle_grid.is_empty() {
        return 0;
    }

    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    if n == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = if obstacle_grid[0][0] == 1 { 0 } else { 1 };

    for r in 0..m {
        for c in 0..n {
            if obstacle_grid[r][c] == 1 {
                continue;
            }
            if r > 0 {
                dp[r][c] += dp[r - 1][c];
            }
            if c > 0 {
                dp[r][c] += dp[r][c - 1];
            }
        }
    }

    dp[m - 1][n - 1]
}

// Space-optimized DP.
// dp[c] = number of ways to reach column c in the current row.
//
// Before updating dp[c], it represents ways from the top.
// After updating dp[c - 1], it represents ways from the the left in the current row.
//
// If the current cell is an obstacle, set dp[c] = 0 because not path can stand
// there, and cells below also cannot use it s a path from above.
//
// Time: O(m * n)
// Space: O(n)

pub fn unique_paths_with_obstacles_optmized(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    if obstacle_grid.is_empty() {
        return 0;
    }

    let n = obstacle_grid[0].len();
    if n == 0 {
        return 0;
    }

    let mut dp = vec![0; n];

    dp[0] = if obstacle_grid[0][0] == 1 { 0 } else { 1 };

    for row in &obstacle_grid {
        for c in 0..n {
            if row[c] == 1 {
                dp[c] = 0;
            } else if c > 0 {
                dp[c] += dp[c - 1];
            }
        }
    }

    dp[n - 1]
}
