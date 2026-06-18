// We start from the ocean coasts and climb up marking each cell that can be reached.
// This is more efficient than checking cells on land.
// DFS was used but it's possible to solve with BFS.

// Time: O(n * m), where n is the row count and m is the column count.
// Each cell can be visited at most once from the Pacific side and once from
// the Atlantic side. The final scan is also O(n * m).
// Space: O(n * m), for the two visited grids and the DFS stack.
// Output space can also be O(n * m).

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let height = heights.len();
    let width = heights[0].len();

    let mut atl = vec![vec![false; width]; height];
    let mut pac = vec![vec![false; width]; height];

    for r in 0..height {
        dfs(r, 0, &mut pac, &heights);
        dfs(r, width - 1, &mut atl, &heights);
    }

    for c in 0..width {
        dfs(0, c, &mut pac, &heights);
        dfs(height - 1, c, &mut atl, &heights);
    }

    let mut results = Vec::new();
    for r in 0..height {
        for c in 0..width {
            if atl[r][c] && pac[r][c] {
                results.push(vec![r as i32, c as i32]);
            }
        }
    }

    results
}

fn dfs(start_r: usize, start_c: usize, visited: &mut [Vec<bool>], grid: &[Vec<i32>]) {
    if visited[start_r][start_c] {
        return;
    }

    // This can be reused among calls as I've did in other examples.
    let mut stack = vec![(start_r, start_c)];
    visited[start_r][start_c] = true;

    let height = grid.len();
    let width = grid[0].len();
    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some((cur_r, cur_c)) = stack.pop() {
        let cur_val = grid[cur_r][cur_c];
        for (dr, dc) in DIRS {
            let Some(row) = cur_r.checked_add_signed(dr) else {
                continue;
            };
            let Some(col) = cur_c.checked_add_signed(dc) else {
                continue;
            };
            if row >= height || col >= width {
                continue;
            }

            if visited[row][col] || grid[row][col] < cur_val {
                continue;
            }

            visited[row][col] = true;
            stack.push((row, col));
        }
    }
}
