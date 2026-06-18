// Prefix Tree (Trie) using HashMap as it's the standard implementation for
// Tries for all input kinds.

// I've provided another implementation for the spacial use case in this example.

// Time: O(L) as L = word.len() for each operation
// Space: O(T) where T is the number of tree nodes

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PrefixTree {
    root: TrieNode,
}

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    end_of_word: bool,
}

impl PrefixTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;

        for ch in word.chars() {
            cur = cur.children.entry(ch).or_default();
        }

        cur.end_of_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        self.find_node(word.as_str()).is_some_and(|n| n.end_of_word)
    }

    pub fn starts_with(&self, word: String) -> bool {
        self.find_node(word.as_str()).is_some()
    }

    fn find_node(&self, word: &str) -> Option<&TrieNode> {
        let mut cur = &self.root;

        for ch in word.chars() {
            cur = cur.children.get(&ch)?;
        }

        Some(cur)
    }
}
