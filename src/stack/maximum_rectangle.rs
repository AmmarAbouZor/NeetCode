// NOTE: Problem exists on LeetCode only: https://leetcode.com/problems/maximal-rectangle/

// Convert each row into a histogram and run Largest Rectangle in Histogram.
//
// `heights[c]` = number of consecutive '1's ending at the current row in column c.
// For each row, the largest all-1 rectangle ending at that row is the largest
// rectangle in this histogram.
//
// Time: O(rows * cols)
// Space: O(cols), for heights and stack.
pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut max_area = 0_usize;
    let mut heights = vec![0; cols];

    let mut stack = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == '1' {
                heights[c] += 1;
            } else {
                heights[c] = 0;
            }
        }

        let area = largest_histogram_area(&heights, &mut stack);

        max_area = max_area.max(area);
    }

    max_area as i32
}

// Monotonic increasing stack of histogram bar indices.
// A sentinel 0 height flushes all remaining bars at the end.
fn largest_histogram_area(heights: &[usize], stack: &mut Vec<usize>) -> usize {
    stack.clear();
    let mut max_area = 0;

    for (idx, cur_height) in heights
        .iter()
        .copied()
        .chain(std::iter::once(0))
        .enumerate()
    {
        while let Some(top) = stack.pop_if(|j| cur_height < heights[*j]) {
            // `idx` is the first shorter bar on the right.
            // `left` is the nearest shorter bar on the left.
            // The rectangle can only use bars between them: left + 1..idx - 1.
            let width = if let Some(&left) = stack.last() {
                idx - left - 1
            } else {
                idx
            };

            let area = width * heights[top];
            max_area = max_area.max(area);
        }

        stack.push(idx);
    }

    max_area
}
