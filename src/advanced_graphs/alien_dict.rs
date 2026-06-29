use std::collections::{HashMap, HashSet, VecDeque};

// Alien Dictionary = topological sort over characters.
//
// The input words are sorted according to an unknown alphabet. For each adjacent
// pair of words, the first position where they differ gives one ordering rule:
// word1[i] must come before word2[i].
//
// Only the first difference matters. After that point, the dictionary order is
// already decided, so later characters do not give reliable constraints.
//
// Build a directed graph:
// edge a -> b means character `a` must appear before character `b`.
//
// Then run Kahn's BFS topological sort using indegrees. Characters with indegree
// 0 have no remaining prerequisites and can be placed next in the result.
//
// Kahn's algorithm:
// Start with all nodes that have indegree 0, meaning they have no remaining
// prerequisites. Pop one node, add it to the result, and remove its outgoing
// edges by decrementing each neighbor's indegree. When a neighbor reaches
// indegree 0, it becomes ready to process.
//
// If a cycle exists, nodes in that cycle never reach indegree 0, so the final
// result will contain fewer nodes than the graph.
//
// Invalid cases:
// - prefix conflict: "abc" appears before "ab"
// - cycle in character dependencies, detected when topo sort cannot include all chars
//
// Time: O(C + E), where C is total characters across all words and E is number
// of ordering edges. With lowercase English letters, E is bounded by a constant.
// Space: O(V + E), where V is number of unique characters.

pub fn foreign_dictionary(words: Vec<String>) -> String {
    let mut graph: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut indegree: HashMap<u8, i32> = HashMap::new();

    // Add every character as a node, even if it has no edges.
    for word in &words {
        for &ch in word.as_bytes() {
            graph.entry(ch).or_default();
            indegree.entry(ch).or_insert(0);
        }
    }

    // Infer ordering edges from adjacent word pairs.
    for win in words.windows(2) {
        let word1 = win[0].as_bytes();
        let word2 = win[1].as_bytes();

        let min_len = word1.len().min(word2.len());
        let mut found_diff = false;

        for i in 0..min_len {
            let from = word1[i];
            let to = word2[i];
            if from != to {
                let neighbors = graph.get_mut(&from).unwrap();
                if neighbors.insert(to) {
                    *indegree.get_mut(&to).unwrap() += 1;
                }

                found_diff = true;
                break;
            }
        }

        // Invalid prefix case: longer word cannot come before its own prefix.
        if !found_diff && word1.len() > word2.len() {
            return String::new();
        }
    }

    // Kahn's BFS topological sort using indegrees
    let mut queue = VecDeque::new();

    // Start with characters that have no prerequisites.
    for (&ch, &degree) in indegree.iter() {
        if degree == 0 {
            queue.push_back(ch);
        }
    }

    let mut res = String::new();

    while let Some(ch) = queue.pop_front() {
        res.push(ch as char);

        if let Some(neighbors) = graph.get(&ch) {
            for &next in neighbors {
                let degree = indegree.get_mut(&next).unwrap();
                *degree -= 1;

                if *degree == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    if res.len() == indegree.len() {
        res
    } else {
        String::new()
    }
}
