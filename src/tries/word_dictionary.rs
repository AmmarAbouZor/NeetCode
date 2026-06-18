// Word Dictionary with support for wild cards using Tries
// Trie is special case matching the input of lower case ascii chars

// add_word: O(L)
// search: O(26^D * L) worst case, where D is number of '.' wildcards.
// In the absolute worst case, this can visit every trie node: O(T).
// Space: O(T * 26) ~= O(T), where T is number of trie nodes.

#[derive(Debug, Default)]
pub struct WordDictionary {
    root: TrieNode,
}

#[derive(Debug, Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    end_of_word: bool,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_word(&mut self, word: String) {
        let mut cur = &mut self.root;
        for ch in word.as_bytes() {
            let idx = (ch - b'a') as usize;
            cur = cur.children[idx]
                .get_or_insert_with(|| Box::new(TrieNode::default()))
                .as_mut();
        }

        cur.end_of_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::dfs(&self.root, word.as_bytes(), 0)
    }

    fn dfs(mut cur: &TrieNode, word: &[u8], idx: usize) -> bool {
        for j in idx..word.len() {
            if word[j] == b'.' {
                for child in cur.children.iter().filter_map(|ch| ch.as_ref()) {
                    if Self::dfs(child, word, j + 1) {
                        return true;
                    }
                }
                return false;
            } else {
                let i = (word[j] - b'a') as usize;
                cur = match &cur.children[i] {
                    Some(child) => child,
                    None => return false,
                }
            }
        }
        cur.end_of_word
    }
}
