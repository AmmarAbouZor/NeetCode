// Backtracking with a trie.
//
// Build one trie for all words, then start DFS from each board cell. The trie
// lets us stop early when the current path is not a prefix of any remaining word.
//
// The board is marked temporarily with `#` to avoid a separate visited grid.
//
// When a word is found, `take()` removes it from the trie node so it is not
// returned again. After DFS, empty trie branches are pruned to reduce later work.

// Build: O(total word chars).
// Search worst case: O(rows * cols * 4^L), with trie pruning in practice.
// Space: O(total word chars) for the trie and O(L) recursion depth.

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    word: Option<String>,
}

impl TrieNode {
    pub fn add_word(&mut self, word: String) {
        let mut cur = self;
        for b in word.as_bytes() {
            let idx = (b - b'a') as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(TrieNode::default()));
        }

        cur.word = Some(word);
    }

    pub fn is_empty(&self) -> bool {
        let Self { children, word } = self;
        word.is_none() && children.iter().all(|c| c.is_none())
    }
}

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut board = board;
    let mut trie = TrieNode::default();

    for word in words {
        trie.add_word(word);
    }

    let rows = board.len();
    let cols = board[0].len();
    let mut res = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            dfs(&mut board, r, c, &mut trie, &mut res);
        }
    }

    res
}

fn dfs(board: &mut [Vec<char>], r: usize, c: usize, node: &mut TrieNode, res: &mut Vec<String>) {
    let ch = board[r][c];

    if ch == '#' {
        return;
    }

    let idx = (ch as u8 - b'a') as usize;

    let Some(next) = node.children[idx].as_mut() else {
        return;
    };

    // Remove the word so it is not returned again.
    if let Some(word) = next.word.take() {
        res.push(word);
    }

    // Mark this board cell as used on the current path.
    board[r][c] = '#';

    if r > 0 {
        dfs(board, r - 1, c, next, res);
    }

    if r < board.len() - 1 {
        dfs(board, r + 1, c, next, res);
    }

    if c > 0 {
        dfs(board, r, c - 1, next, res);
    }

    if c < board[0].len() - 1 {
        dfs(board, r, c + 1, next, res);
    }

    // Prune exhausted trie branch.
    if next.is_empty() {
        node.children[idx] = None;
    }

    // Reset char value after finishing backtracking.
    board[r][c] = ch;
}
