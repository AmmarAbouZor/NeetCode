// Backtracking from each cell in each direction.

// Time: O(R * C * 4 * 3^(L - 1)), where R and C are board dimensions
// and L is word.len(). We start from every cell. From each start, the first
// step can branch up to 4 directions, then each later step has at most 3
// choices because we cannot revisit the previous cell.
//
// Space: O(R * C + L): visited matrix is O(R * C), recursion stack is O(L),
// and collecting the word chars is O(L).

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();
    let height = board.len();
    let width = board[0].len();

    let mut visited = vec![vec![false; width]; height];

    for r in 0..height {
        for c in 0..width {
            if dfs(r, c, &board, &word, &mut visited) {
                return true;
            }
        }
    }

    false
}

fn dfs(
    row: usize,
    col: usize,
    board: &[Vec<char>],
    word: &[char],
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    if visited[row][col] {
        return false;
    }

    if word[0] != board[row][col] {
        return false;
    }

    if word.len() == 1 {
        return true;
    }

    visited[row][col] = true;

    let next_word = &word[1..];

    // Up
    if row > 0 && dfs(row - 1, col, board, next_word, visited) {
        return true;
    }

    // Down
    if row < board.len() - 1 && dfs(row + 1, col, board, next_word, visited) {
        return true;
    }

    // Left
    if col > 0 && dfs(row, col - 1, board, next_word, visited) {
        return true;
    }

    // Right
    if col < board[0].len() - 1 && dfs(row, col + 1, board, next_word, visited) {
        return true;
    }

    // It's OK to return early above on true as it will bubble up and we don't need to
    // do further checks after that point.
    visited[row][col] = false;

    false
}
