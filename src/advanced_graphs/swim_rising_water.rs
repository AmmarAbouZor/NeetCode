use std::{cmp::Reverse, collections::BinaryHeap};

// Dijkstra / minimum bottleneck path.
//
// The cost of a path is the maximum elevation seen on that path, not the sum of
// edge costs. So when moving to a neighbor:
//
// next_time = max(current_time, neighbor_elevation)
//
// The min-heap always expands in the reachable cell with the smallest required
// time so far. The first time we pop the bottom-right cell, that time is optimal.
//
// Time: O(N^2 LogN) for an n * n grid
// Space: O(N^2) for visited and the heap.

pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    // Matrix is square
    let n = grid.len();

    let mut visited = vec![vec![false; n]; n];

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((grid[0][0], 0, 0)));

    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some(Reverse((time, r, c))) = heap.pop() {
        if visited[r][c] {
            continue;
        }

        if r == n - 1 && c == n - 1 {
            return time;
        }

        visited[r][c] = true;

        for (dr, dc) in DIRS {
            let Some(next_r) = r.checked_add_signed(dr) else {
                continue;
            };
            let Some(next_c) = c.checked_add_signed(dc) else {
                continue;
            };

            if next_r >= n || next_c >= n {
                continue;
            }

            if visited[next_r][next_c] {
                continue;
            }

            let next_time = time.max(grid[next_r][next_c]);

            heap.push(Reverse((next_time, next_r, next_c)));
        }
    }

    -1
}
