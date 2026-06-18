// Prefix Tree (Trie) for words implemented using arrays as the input
// is lower case ascii chars only.

// Time: O(L) for each function where L is the length of the word.
// Space: O(26 * T) ~= O(T) where T is the count of tree nodes in the tree

#[derive(Debug, Default)]
pub struct PrefixTree {
    root: TrieNode,
}

#[derive(Debug, Default)]
pub struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    end_of_word: bool,
}

impl PrefixTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: String) {
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
        self.find_node(word.as_str()).is_some_and(|n| n.end_of_word)
    }

    pub fn starts_with(&self, word: String) -> bool {
        self.find_node(word.as_str()).is_some()
    }

    fn find_node(&self, word: &str) -> Option<&TrieNode> {
        let mut cur = &self.root;

        for &ch in word.as_bytes() {
            let idx = (ch - b'a') as usize;
            cur = cur.children[idx].as_ref()?;
        }

        Some(cur)
    }
}
