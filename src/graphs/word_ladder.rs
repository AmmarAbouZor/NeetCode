use std::collections::{HashSet, VecDeque};

// BFS shortest path in an implicit graph.
//
// Each word is a node. Two words have an edge if they differ by one character.
// From each word, generate all one-character mutations and keep the ones that exists
// in words.
//
// BFS level count is the transformation length. Removing a word from `words`
// marks it visited and prevents cycles.
//
// Note:
// It's possible to improve this solution by starting from both start and end nodes
// and meet in the middle.
//
// Time: O(N * L * 26 *  L) including hashing, often simplified to O(N * L * 26).
// Space: O(N * L)

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut words: HashSet<Vec<u8>> = word_list.into_iter().map(|w| w.into_bytes()).collect();
    let begin_word = begin_word.into_bytes();
    let end_word = end_word.into_bytes();

    if !words.contains(&end_word) {
        return 0;
    }

    let mut queue = VecDeque::new();
    queue.push_back(begin_word);

    let mut count = 0;

    while !queue.is_empty() {
        count += 1;
        let len = queue.len();

        for _ in 0..len {
            let mut word = queue.pop_front().unwrap();

            if word == end_word {
                return count;
            }

            for idx in 0..word.len() {
                let original_ch = word[idx];

                for ch in b'a'..=b'z' {
                    if ch == original_ch {
                        continue;
                    }

                    word[idx] = ch;

                    if words.remove(&word) {
                        queue.push_back(word.clone());
                    }
                }

                word[idx] = original_ch;
            }
        }
    }

    0
}

// Bidirectional BFS follow-up
//
// Keep one frontier from `begin_word` and one from `end_word`. At each level,
// expand the smaller frontier to reduce branching. When a generated mutation
// exists in the opposite frontier, the searches meet and we return steps + 1.
//
// `words` stores unvisited dictionary words. Removing a word when it enters a
// frontier prevents revisiting it from either side.
//
// Same worst-case Big-O as normal BFS, but usually much faster in practice
// because it searches roughly half of depth from both ends.
//
// Time: O(N * L * 26 * L) including hashing. often simplified to O(N * L * 26).
// Space: O(N * L)
pub fn ladder_length_bidirectional(
    begin_word: String,
    end_word: String,
    word_list: Vec<String>,
) -> i32 {
    let mut words: HashSet<Vec<u8>> = word_list.into_iter().map(|w| w.into_bytes()).collect();
    let begin_word = begin_word.into_bytes();
    let end_word = end_word.into_bytes();

    if !words.contains(&end_word) {
        return 0;
    }

    let mut begin_set = HashSet::new();
    begin_set.insert(begin_word);

    let mut end_set = HashSet::new();
    end_set.insert(end_word.clone());

    // Prevent revisiting the end word through the dictionary side.
    words.remove(&end_word);

    let mut steps = 1;

    while !begin_set.is_empty() && !end_set.is_empty() {
        // Always expand the smaller frontier.
        if begin_set.len() > end_set.len() {
            std::mem::swap(&mut begin_set, &mut end_set);
        }

        let mut next_set = HashSet::new();

        for word in begin_set {
            let mut candidate = word;

            for idx in 0..candidate.len() {
                let original = candidate[idx];

                for ch in b'a'..=b'z' {
                    if ch == original {
                        continue;
                    }

                    candidate[idx] = ch;

                    if end_set.contains(&candidate) {
                        return steps + 1;
                    }

                    if words.remove(&candidate) {
                        next_set.insert(candidate.clone());
                    }
                }

                candidate[idx] = original;
            }
        }

        begin_set = next_set;
        steps += 1;
    }

    0
}
