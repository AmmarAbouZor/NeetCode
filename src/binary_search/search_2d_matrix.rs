// O(log(n * m)) as we flattened the matrix and applied binary search on it
// O(1) as we flatten the matrix in place by calculating row and col from the combined index

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut l = 0;
    let mut r = n * m;

    while l < r {
        let mid = l + (r - l) / 2;

        // Calculate row and col from combined index
        let mid_row = mid / m;
        let mid_col = mid % m;

        use std::cmp::Ordering;
        match matrix[mid_row][mid_col].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => l = mid + 1,
            Ordering::Greater => r = mid,
        }
    }

    false
}
