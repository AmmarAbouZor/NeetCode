//! O(n2) as we need to go through each item.
//! The hardest point is deal with the squares but this can be memorized as
//! Sudoku can't change much.
//! Note:
//! The bit mask solution is also important making this a good candidate
//! for a Rust interview

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;

    // Have a set for each row, col and square.
    // This will help checking on all of them in one nested loop.
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut squares = vec![HashSet::new(); 9];

    for r in 0..9 {
        for c in 0..9 {
            let ch = board[r][c];
            if ch == '.' {
                continue;
            }

            // Important formula to understand & memorize
            let square_idx = (r / 3) * 3 + c / 3;

            if !rows[r].insert(ch) || !cols[c].insert(ch) || !squares[square_idx].insert(ch) {
                return false;
            }
        }
    }

    true
}

/// This one is the same but it uses bit-masks.
/// This could be a good candidate for my interview
pub fn is_valid_sudoku_bitmask(board: Vec<Vec<char>>) -> bool {
    // Have a bit-mask for each row, col and square.
    // This will help checking on all of them in one nested loop.
    let mut rows = [0_u32; 9];
    let mut cols = [0_u32; 9];
    let mut squares = [0_u32; 9];

    for r in 0..9 {
        for c in 0..9 {
            let ch = board[r][c];
            if ch == '.' {
                continue;
            }

            let val = ch as u32 - 'a' as u32;
            let bit = 1 << val;

            // Important formula to understand & memorize
            let square_idx = (r / 3) * 3 + c / 3;

            // Check using bit-mask
            if (rows[r] & bit) != 0 || (cols[c] & bit) != 0 || (squares[square_idx] & bit) != 0 {
                return false;
            }

            // Add bit to the masks.
            rows[r] |= bit;
            cols[c] |= bit;
            squares[square_idx] |= bit;
        }
    }

    true
}
