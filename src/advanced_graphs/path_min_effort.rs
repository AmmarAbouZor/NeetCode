use std::{cmp::Reverse, collections::BinaryHeap};

// Dijkstra for a minimum bottleneck path.
//
// `diffs[r][c]` is the minimum effort needed to reach cell (r, c).
// Effort is the maximum absolute height difference on the path, not the sum.
//
// Time: O(rows * cols * log(rows * cols))
// Space: O(rows * cols)

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    if heights.is_empty() || heights[0].is_empty() {
        return 0;
    }

    let rows = heights.len();
    let cols = heights[0].len();

    let mut diffs = vec![vec![i32::MAX; cols]; rows];

    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    diffs[0][0] = 0;
    heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((cur_diff, row, col))) = heap.pop() {
        if cur_diff > diffs[row][col] {
            continue;
        }

        if row == rows - 1 && col == cols - 1 {
            return cur_diff;
        }

        const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

        for (dr, dc) in DIRS {
            let Some(next_r) = row.checked_add_signed(dr) else {
                continue;
            };
            let Some(next_c) = col.checked_add_signed(dc) else {
                continue;
            };

            if next_r >= rows || next_c >= cols {
                continue;
            }

            let diff = (heights[row][col] - heights[next_r][next_c]).abs();
            let best_diff = cur_diff.max(diff);

            if best_diff < diffs[next_r][next_c] {
                diffs[next_r][next_c] = best_diff;
                heap.push(Reverse((best_diff, next_r, next_c)));
            }
        }
    }

    unreachable!("There is a way to reach the target.");
}
