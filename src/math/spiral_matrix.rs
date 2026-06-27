// The idea is to define the boundary `left, top, right, bottom` and shift them
// inside after each iteration.
// Each loop processes four sides in order: top row, right column, bottom row, left column.
// We must recheck conditions inside the loop as values are changing.

// Time: O(n * m)
// Space: O(1) excluding results. With results: O(n * m)

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let m = matrix.len();
    let n = matrix[0].len();

    let mut res = Vec::with_capacity(n * m);

    let mut top = 0;
    let mut bottom = m - 1;
    let mut left = 0;
    let mut right = n - 1;

    while top <= bottom && left <= right {
        for c in left..=right {
            res.push(matrix[top][c]);
        }
        top += 1;

        for r in top..=bottom {
            res.push(matrix[r][right]);
        }
        if right == 0 {
            break;
        }
        right -= 1;

        if top <= bottom {
            for c in (left..=right).rev() {
                res.push(matrix[bottom][c]);
            }

            if bottom == 0 {
                break;
            }
            bottom -= 1;
        }

        if left <= right {
            for r in (top..=bottom).rev() {
                res.push(matrix[r][left]);
            }

            left += 1;
        }
    }

    res
}
