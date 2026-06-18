// We start at the edges of the board and mark all 'O' cells as those cells will
// not change. Then finally we iterate through the cells and change 'O' cells which
// aren't marked to 'X'

// Time: O(n * m): each cell is marked at most once, plus a final scan.
// Space O(n * m): For visited grid and DFS Stack in worst case.

// Optimizations:
// * It's possible to avoid allocating visited grid by marking border cells temporally
//   with 'T' then it can converted back at the end
// * Stack memory can be cached and reused as previous solutions.

pub fn solve(board: &mut [Vec<char>]) {
    if board.is_empty() || board[0].is_empty() {
        return;
    }

    let height = board.len();
    let width = board[0].len();

    let mut visited = vec![vec![false; width]; height];

    for r in 0..height {
        dfs(r, 0, board, &mut visited);
        dfs(r, width - 1, board, &mut visited);
    }

    for c in 0..width {
        dfs(0, c, board, &mut visited);
        dfs(height - 1, c, board, &mut visited);
    }

    for r in 0..height {
        for c in 0..width {
            if board[r][c] == 'O' && !visited[r][c] {
                board[r][c] = 'X';
            }
        }
    }
}

fn dfs(start_r: usize, start_c: usize, board: &[Vec<char>], visited: &mut [Vec<bool>]) {
    if board[start_r][start_c] != 'O' || visited[start_r][start_c] {
        return;
    }

    let mut stack = vec![(start_r, start_c)];
    visited[start_r][start_c] = true;

    let height = board.len();
    let width = board[0].len();
    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    while let Some((cur_r, cur_c)) = stack.pop() {
        for (dr, dc) in DIRS {
            let Some(row) = cur_r.checked_add_signed(dr) else {
                continue;
            };
            let Some(col) = cur_c.checked_add_signed(dc) else {
                continue;
            };

            if row >= height || col >= width {
                continue;
            }

            if visited[row][col] || board[row][col] != 'O' {
                continue;
            }

            visited[row][col] = true;
            stack.push((row, col));
        }
    }
}
