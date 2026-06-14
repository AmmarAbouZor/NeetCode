pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    if s.is_empty() {
        return true;
    }

    // Fill the character counts maps.
    use std::collections::HashMap;

    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();

    fill_counts(s.as_str(), &mut s_map);
    fill_counts(t.as_str(), &mut t_map);

    // Compare the maps
    if s_map.len() != t_map.len() {
        return false;
    }

    for (ch, count_s) in s_map.into_iter() {
        let Some(count_t) = t_map.get(&ch) else {
            return false;
        };

        if *count_t != count_s {
            return false;
        }
    }

    true
}

fn fill_counts(s: &str, map: &mut std::collections::HashMap<char, usize>) {
    s.chars().for_each(|char| {
        *map.entry(char).or_insert(0) += 1;
    });
}

// This solution is using one map only
pub fn is_anagram_one_map(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    if s.is_empty() {
        return true;
    }

    // Fill character counts for first string only.
    use std::collections::HashMap;

    let mut char_counts = HashMap::new();

    s.chars().for_each(|ch| {
        char_counts
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

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
