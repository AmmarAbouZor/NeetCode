// Graph traversal using BFS. This can also be solved with DFS using a stack.

// Time O(n * m) where n is rows count and m is columns count
// Space: O(n * m), for visited plus the BFS queue in the worst case.

use std::collections::VecDeque;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let height = grid.len();
    let width = grid[0].len();

    // Create the visited matrix. For the little input it's better to
    // avoid hash computations
    let mut visited = vec![vec![false; width]; height];

    // Reuse same memory instead of allocating it on each bfs call.
    let mut queue_cache = VecDeque::new();

    let mut islands = 0;
    for r in 0..height {
        for c in 0..width {
            if grid[r][c] != '1' || visited[r][c] {
                continue;
            }

            islands += 1;
            bfs(r, c, &grid, &mut visited, &mut queue_cache)
        }
    }

    islands
}

fn bfs(
    row: usize,
    col: usize,
    grid: &[Vec<char>],
    visited: &mut [Vec<bool>],
    queue: &mut VecDeque<(usize, usize)>,
) {
    queue.clear();
    queue.push_back((row, col));
    visited[row][col] = true;

    let height = grid.len();
    let width = grid[0].len();
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    while let Some((cur_r, cur_c)) = queue.pop_front() {
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

            if visited[r][c] || grid[r][c] != '1' {
                continue;
            }

            visited[r][c] = true;
            queue.push_back((r, c));
        }
    }
}
