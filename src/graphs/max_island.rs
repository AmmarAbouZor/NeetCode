// Graph traversal with DFS. BFS also works

// Time: O(n * m), where n and m are the row and column counts.
// Space: O(n * m), for the visited grid and DFS stack.

// Note: It's possible to avoid allocating memory for visited by changing
// the grid replacing 1 with 2 or other value. However this isn't needed as
// the input size isn't too large.

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut visited = vec![vec![false; width]; height];

    let mut dfs_cache = Vec::new();

    let mut max_area = 0;

    for r in 0..height {
        for c in 0..width {
            if grid[r][c] != 1 || visited[r][c] {
                continue;
            }

            let area = dfs(r, c, &grid, &mut visited, &mut dfs_cache);
            max_area = max_area.max(area);
        }
    }

    max_area as i32
}

fn dfs(
    row: usize,
    col: usize,
    grid: &[Vec<i32>],
    visited: &mut [Vec<bool>],
    stack: &mut Vec<(usize, usize)>,
) -> usize {
    stack.clear();
    stack.push((row, col));
    visited[row][col] = true;

    // We already have the first cell
    let mut area = 1;

    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let height = grid.len();
    let width = grid[0].len();
    while let Some((cur_r, cur_c)) = stack.pop() {
        for (dr, dc) in DIRECTIONS {
            let Some(r) = cur_r.checked_add_signed(dr) else {
                continue;
            };

            let Some(c) = cur_c.checked_add_signed(dc) else {
                continue;
            };

            if r >= height || c >= width {
                continue;
            }

            if visited[r][c] || grid[r][c] != 1 {
                continue;
            }

            visited[r][c] = true;
            area += 1;
            stack.push((r, c));
        }
    }

    area
}
