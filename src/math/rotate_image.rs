// This is special case of rotation where we don't need to apply normal rotation formulas.
// Instead we split the rotation into two steps: Reverse and transpose. We can do it both orders:
// * Reverse matrix vertically then transpose
// * Transpose then reverse each row from the matrix.

// Assumes matrix is n x n.
// Time: O(n^2)
// Space: O(1)

// Reverse then transpose
pub fn rotate(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();

    // Reverse matrix vertically
    matrix.reverse();

    // Transpose
    for i in 0..n {
        for j in i + 1..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
}

// Transpose then reverse each row
pub fn rotate2(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();

    // Transpose matrix first
    for i in 0..n {
        for j in i + 1..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    // Now reverse each row
    for row in matrix.iter_mut() {
        row.reverse();
    }
}
