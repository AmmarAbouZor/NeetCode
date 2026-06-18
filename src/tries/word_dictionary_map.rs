// Standard Word Dictionary implementation with support of wild cards.
// Uses HashMap to support all char kinds

// Time for add_word O(L) where L is word length
// Time for search O(26^D * L) where D is num of wild cards.
// search: O(A^D * L) worst case, where D is number of wildcards
// and A is the maximum branching factor. It can also be described as O(T)
// in the worst case because search may visit every trie node.

// Space O(T) where T is number of Trie nodes.
// Space Search O(L) extra space for collecting word into chars

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct WordDictionary {
    root: TrieNode,
}

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    end_of_word: bool,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_word(&mut self, word: String) {
        let mut cur = &mut self.root;
        for ch in word.chars() {
            cur = cur.children.entry(ch).or_default();
        }
        cur.end_of_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        Self::dfs(&self.root, &word, 0)
    }

    fn dfs(mut cur: &TrieNode, word: &[char], idx: usize) -> bool {
        for j in idx..word.len() {
            if word[j] == '.' {
                for child in cur.children.values() {
                    if Self::dfs(child, word, j + 1) {
                        return true;
                    }
                }
                return false;
            } else {
                cur = match cur.children.get(&word[j]) {
                    Some(child) => child,
                    None => return false,
                }
            }
        }

        cur.end_of_word
    }
}
