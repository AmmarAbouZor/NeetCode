// This is a multi-source BFS problem like Islands and Treasure.
// This solution counts each BFS wave as one minute.
// We also track the fresh fruit count to detect cases where not all fruit can rot.

// Time: O(n * m), where n and m are the row and column counts.
// Space: O(n * m), for the BFS queue in the worst case.

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let mut grid = grid;

    let mut queue = std::collections::VecDeque::new();
    let mut fresh_count = 0;

    for (r_idx, row) in grid.iter().enumerate() {
        for (c_idx, num) in row.iter().enumerate() {
            match num {
                1 => fresh_count += 1,
                2 => queue.push_back((r_idx, c_idx)),
                _ => {}
            }
        }
    }

    let mut time = 0;
    let height = grid.len();
    let width = grid[0].len();
    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while fresh_count > 0 && !queue.is_empty() {
        let q_len = queue.len();
        for _ in 0..q_len {
            let Some((cur_r, cur_c)) = queue.pop_front() else {
                break;
            };
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
                if grid[row][col] != 1 {
                    continue;
                }

                grid[row][col] = 2;
                fresh_count -= 1;
                queue.push_back((row, col));
            }
        }
        time += 1;
    }

    if fresh_count > 0 { -1 } else { time }
}

// This is the same as the one above. However the code is more verbose than this preferred one.
// Also it's not clear that we are counting on each wave as we are saving the values in the grid
// itself, exactly to what we've done in treasure solution.
pub fn oranges_rotting_longer(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let mut grid = grid;

    let mut queue = std::collections::VecDeque::new();
    let mut fresh_count = 0;

    for (r_idx, row) in grid.iter().enumerate() {
        for (c_idx, num) in row.iter().enumerate() {
            match num {
                1 => fresh_count += 1,
                2 => queue.push_back((r_idx, c_idx)),
                _ => {}
            }
        }
    }

    if fresh_count == 0 {
        return 0;
    }

    if queue.is_empty() {
        return -1;
    }

    let mut max_days = 2;
    let height = grid.len();
    let width = grid[0].len();

    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while let Some((cur_r, cur_c)) = queue.pop_front() {
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

            if grid[row][col] != 1 {
                continue;
            }

            fresh_count -= 1;
            let val = grid[cur_r][cur_c] + 1;
            grid[row][col] = val;
            max_days = max_days.max(val);

            queue.push_back((row, col));
        }
    }

    if fresh_count > 0 { -1 } else { max_days - 2 }
}
