// 2D Dynamic programming using DP as grid.
// It feels like a 2D version of climbing stairs as each cell can be reached from
// either left or above.

// Understand the full grid solution first, then use the row-optimized version.

// The optimized solution uses one row instead of the whole grid.

// Time: O(m * n) as we go through each cell
// Space: O(n) for the row.

// Initialize values to 1 and iterate from 1 instead of 0 to avoid
// underflow checks and make code shorter.
pub fn unique_paths_row_short(m: i32, n: i32) -> i32 {
    if m <= 0 || n <= 0 {
        return 0;
    }

    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![1; n];

    for _r in 1..m {
        for c in 1..n {
            // dp[c] is still the value from the cell above before we overwrite it.
            // dp[c] = dp[c] + dp[c -1];
            dp[c] += dp[c - 1];
        }
    }

    dp[n - 1]
}

pub fn unique_paths_row_longer(m: i32, n: i32) -> i32 {
    if m <= 0 || n <= 0 {
        return 0;
    }

    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![0; n];
    dp[0] = 1;

    for _r in 0..m {
        for c in 0..n {
            if let Some(left) = c.checked_sub(1) {
                // dp[c] is still the value from the cell above before we overwrite it.
                // dp[c] = dp[c] + dp[left];
                dp[c] += dp[left];
            }
        }
    }

    dp[n - 1]
}

// Time: O(m * n) as we go through each cell
// Space: O(m * n) for dp grid.

// Shorter version than the one below initializing the grid to ones and iterating
// from 1 instead of 0 to avoid underflow checks.
pub fn unique_paths_grid_short(m: i32, n: i32) -> i32 {
    if m <= 0 || n <= 0 {
        return 0;
    }

    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![vec![1; n]; m];

    for r in 1..m {
        for c in 1..n {
            dp[r][c] = dp[r - 1][c] + dp[r][c - 1];
        }
    }

    dp[m - 1][n - 1]
}

pub fn unique_paths_grid_long(m: i32, n: i32) -> i32 {
    if m <= 0 || n <= 0 {
        return 0;
    }

    let m = m as usize;
    let n = n as usize;

    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = 1;

    for r in 0..m {
        for c in 0..n {
            if let Some(up) = r.checked_sub(1) {
                dp[r][c] += dp[up][c];
            }
            if let Some(left) = c.checked_sub(1) {
                dp[r][c] += dp[r][left];
            }
        }
    }

    dp[m - 1][n - 1]
}
