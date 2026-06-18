// The trick for this questions is to start from the treasure themselves
// with BFS and do that at the same time.
// This will guarantee that land cells will be reached from the nearest treasure
// as BFS move in wave like in all directions each level.

// Better wording:
// Start BFS from all treasures at once. Multi-source BFS guarantees each land
// cell is filled by the nearest treasure because BFS expands in distance order.

// Time: O(n * m), where n and m are the row and column counts.
// Space: O(n * m), for the BFS queue in the worst case.

pub fn islands_and_treasure(grid: &mut [Vec<i32>]) {
    if grid.is_empty() || grid[0].is_empty() {
        return;
    }

    let mut queue = std::collections::VecDeque::new();

    for (r_idx, row) in grid.iter().enumerate() {
        for (c_idx, &num) in row.iter().enumerate() {
            if num == 0 {
                queue.push_back((r_idx, c_idx));
            }
        }
    }

    let height = grid.len();
    let width = grid[0].len();

    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some((cur_r, cur_c)) = queue.pop_front() {
        for (dr, dc) in DIRECTIONS {
            let Some(row) = cur_r.checked_add_signed(dr) else {
                continue;
            };
            let Some(col) = cur_c.checked_add_signed(dc) else {
                continue;
            };

            if row >= height || col >= width {
                continue;
            }
            // We are interested only in the yet not reached cells
            // Water cells or cells that has been already resolved are ignored.
            if grid[row][col] != i32::MAX {
                continue;
            }

            grid[row][col] = grid[cur_r][cur_c] + 1;

            queue.push_back((row, col));
        }
    }
}
