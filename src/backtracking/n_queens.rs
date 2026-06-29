use std::collections::HashSet;

// Backtracking row by row.
// Since each recursive level places one queen in one row, we only track used columns
// and diagonals. The set version keeps the diagonal formulas easy to remember:
// main diagonal = r - c, anti diagonal = r + c.
//
// Time: O(n!) with pruning, instead of trying every column in every row.
// Space: O(n^2) for the board, plus O(n) tracking state.
//
// The optimized version below replaces diagonal HashSets with bool arrays.

pub fn solve_n_queens_sets(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;

    let mut board = vec![vec!['.'; n]; n];

    // Used columns.
    let mut cols = vec![false; n];

    // Diagonals are easier to reason about directly with sets.
    let mut main_diag = HashSet::new(); // r - c
    let mut anti_diag = HashSet::new(); // r + c

    let mut res = Vec::new();

    dfs(
        0,
        &mut board,
        &mut cols,
        &mut main_diag,
        &mut anti_diag,
        &mut res,
    );

    res
}

fn dfs(
    r: usize,
    board: &mut [Vec<char>],
    cols: &mut [bool],
    main_diag: &mut HashSet<i32>,
    anti_diag: &mut HashSet<usize>,
    res: &mut Vec<Vec<String>>,
) {
    let n = board.len();
    if r == n {
        let solution: Vec<String> = board.iter().map(|r| r.iter().collect::<String>()).collect();
        res.push(solution);
        return;
    }

    for c in 0..n {
        let main_d_val = r as i32 - c as i32;
        let anti_d_val = r + c;

        if cols[c] || main_diag.contains(&main_d_val) || anti_diag.contains(&anti_d_val) {
            continue;
        }

        cols[c] = true;
        main_diag.insert(main_d_val);
        anti_diag.insert(anti_d_val);
        board[r][c] = 'Q';

        dfs(r + 1, board, cols, main_diag, anti_diag, res);
        cols[c] = false;
        main_diag.remove(&main_d_val);
        anti_diag.remove(&anti_d_val);
        board[r][c] = '.';
    }
}

// Same backtracking as `solve_n_queens_sets`, but uses bool arrays instead of
// HashSets for O(1) tracking with less overhead.
//
// Column index is already `c`. Anti diagonal index is `r + c`.
// Main diagonal `r - c` can be negative, so shift it by `n - 1`:
// `main_d_idx = r + n - 1 - c`.
pub fn solve_n_queens_optimized(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;

    let mut board = vec![vec!['.'; n]; n];

    // Same tracking as the set version, but stored in bool arrays.
    let mut cols = vec![false; n];

    // Shift r - c by n - 1 to avoid negative indices. Range: 0..=2n-2.
    let mut main_diag = vec![false; 2 * n - 1];

    // r + c is already nonnegative. Range: 0..=2n-2.
    let mut anti_diag = vec![false; 2 * n - 1];

    let mut res = Vec::new();

    dfs_opt(
        0,
        &mut board,
        &mut cols,
        &mut main_diag,
        &mut anti_diag,
        &mut res,
    );

    res
}

fn dfs_opt(
    r: usize,
    board: &mut [Vec<char>],
    cols: &mut [bool],
    main_diag: &mut [bool],
    anti_diag: &mut [bool],
    res: &mut Vec<Vec<String>>,
) {
    let n = board.len();
    if r == n {
        let solution: Vec<String> = board.iter().map(|r| r.iter().collect::<String>()).collect();
        res.push(solution);

        return;
    }

    for c in 0..n {
        let main_d_idx = r + n - 1 - c;
        let anti_d_idx = r + c;
        if cols[c] || main_diag[main_d_idx] || anti_diag[anti_d_idx] {
            continue;
        }

        board[r][c] = 'Q';
        cols[c] = true;
        main_diag[main_d_idx] = true;
        anti_diag[anti_d_idx] = true;

        dfs_opt(r + 1, board, cols, main_diag, anti_diag, res);

        board[r][c] = '.';
        cols[c] = false;
        main_diag[main_d_idx] = false;
        anti_diag[anti_d_idx] = false;
    }
}
