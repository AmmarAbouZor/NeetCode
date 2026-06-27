// More simple solution is to scan the matrix and save rows and cols in
// separated lists then zero the items. But this will have space O(m + n).

// To solve it with space O(1) we mark the top row and most left columns with zeros
// for cells equals to zero and then zero the cell in second run.
// However, this will need handling the special case of cell [0][0] as we will be not
// able to determine if we should zero it's row or cols. To solve this we scan the first row
// separately and keep the result in extra boolean while making [0][0] value for first left col only.

// Time: O(n * m)
// Space: O(1)

pub fn set_zeros(matrix: &mut [Vec<i32>]) {
    if matrix.is_empty() || matrix[0].is_empty() {
        return;
    }

    let m = matrix.len();
    let n = matrix[0].len();

    let first_row_zeros = (0..n).any(|c| matrix[0][c] == 0);

    for r in 1..m {
        for c in 0..n {
            if matrix[r][c] == 0 {
                matrix[0][c] = 0;
                matrix[r][0] = 0;
            }
        }
    }

    for r in 1..m {
        for c in 1..n {
            if matrix[0][c] == 0 || matrix[r][0] == 0 {
                matrix[r][c] = 0;
            }
        }
    }

    if matrix[0][0] == 0 {
        (0..m).for_each(|r| matrix[r][0] = 0);
    }

    if first_row_zeros {
        (0..n).for_each(|c| matrix[0][c] = 0);
    }
}
