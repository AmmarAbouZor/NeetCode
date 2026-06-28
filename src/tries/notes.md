# Trie Notes

## When to use a trie

Use a trie for prefix-based string problems:

- insert/search word
- starts_with prefix
- wildcard search
- word dictionary
- word search with prefix pruning

## Node shape

Fixed lowercase ASCII alphabet:

```rust
children: [Option<Box<TrieNode>>; 26]
```

Pros: fast lookup, no hashing. Cons: each node stores 26 slots.

General characters:

```rust
children: HashMap<char, TrieNode>
```

Pros: flexible and sparse. Cons: hashing overhead.

## Basic operations

For word length `L`:

- `insert`: `O(L)` time, `O(L)` new nodes worst case
- `search`: `O(L)` time
- `starts_with`: `O(L)` time

Total space: `O(T)`, where `T` is total trie nodes. Array version is technically `O(26 * T)`, simplified to `O(T)`.

## Shared traversal helper

Use a helper to avoid duplicating search and prefix logic:

```rust
fn find_node(&self, text: &str) -> Option<&TrieNode> {
    let mut cur = &self.root;

    for ch in text.chars() {
        cur = cur.children.get(&ch)?;
    }

    Some(cur)
}
```

For array children:

```rust
let idx = (b - b'a') as usize;
cur = cur.children[idx].as_ref()?;
```

Then:

```rust
search: find_node(word).is_some_and(|node| node.end_of_word)
starts_with: find_node(prefix).is_some()
```

## Insert pattern

Map version:

```rust
cur = cur.children.entry(ch).or_default();
```

Array version:

```rust
cur = cur.children[idx]
    .get_or_insert_with(|| Box::new(TrieNode::default()))
    .as_mut();
```

## Word Dictionary with wildcard `.`

Normal character: follow one child.

Wildcard `.`: try all children recursively.

```rust
if word[i] == '.' {
    for child in cur.children.values() {
        if dfs(child, word, i + 1) {
            return true;
        }
    }
    return false;
}
```

Complexity:

- no wildcard: `O(L)`
- with wildcards: can branch heavily
- worst case: `O(T)` because search may visit every trie node
- loose bound: `O(A^D * L)`, where `A` is max branching factor and `D` is wildcard count

## Word Search II

Use backtracking with a trie.

Build one trie for all words, then DFS from each board cell. Stop early when the current path is not a trie prefix.

Mark visited cells in-place, for example with `'#'`, then restore the char after backtracking.

Store the full word at terminal trie nodes. When found, use `word.take()` so the same word is not returned twice.

After DFS from a child, prune it if it has no word and no children left.

Build: `O(total word chars)`. Search worst case: `O(rows * cols * 4^L)`, with trie pruning in practice. Space: `O(total word chars + L)`.

## Rust notes

- `search` usually takes `&self`, not `&mut self`.
- For Unicode/general `char` traversal, collecting into `Vec<char>` makes indexed DFS easier but costs `O(L)` extra space.
- For LeetCode lowercase ASCII, bytes are simpler and faster.
