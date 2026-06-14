//! NOTE: I've went through the is_anagram for each item as I resolved it
//! before but this ended up with O(N^2 * L). I should have read the contrains as they gave
//! me good hints

/// This will run with O(N * L): where n: number of string, L max length of a string in bytes.
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // This will use the fact that chars in input are all low case english chars.
    // So we can represent the words with an array [usize;26] where the indexes are
    // the byte for each character minux b'a'

    let mut groups = std::collections::HashMap::<[usize; 26], Vec<String>>::new();

    for s in strs {
        let mut counts = [0; 26];

        s.as_bytes().iter().map(|b| *b - b'a').for_each(|b_idx| {
            counts[b_idx as usize] += 1;
        });

        groups.entry(counts).or_default().push(s);
    }

    groups.into_values().collect()
}

/// This will run in O(n^2 * L) where n: num of string , L: max length of string
pub fn group_anagrams_iniffecient(strs: Vec<String>) -> Vec<Vec<String>> {
    // Check special cases
    match strs.len() {
        0 => return Vec::new(),
        1 => return vec![strs],
        _ => {}
    }

    let mut anagrams = std::collections::HashMap::<String, Vec<String>>::new();

    for s in strs {
        let mut found = false;
        for (key, values) in anagrams.iter_mut() {
            if is_anagram(s.as_str(), key) {
                values.push(s.clone());
                found = true;
                break;
            }
        }
        if !found {
            anagrams.insert(s, Vec::new());
        }
    }

    let mut groups = Vec::with_capacity(anagrams.len());

    for (key, values) in anagrams.into_iter() {
        let mut group = Vec::with_capacity(values.len() + 1);
        group.push(key);
        group.extend(values);

        groups.push(group);
    }

    groups
}

fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut char_counts = std::collections::HashMap::new();

    // Fill the char counts map from the first str
    s.chars().for_each(|ch| {
        char_counts
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    // Use the map to validate the other str directly
    for ch in t.chars() {
        let Some(counts) = char_counts.get_mut(&ch) else {
            return false;
        };

        if *counts == 0 {
            return false;
        }

        *counts -= 1;
    }

    char_counts.values().all(|count| *count == 0)
}
