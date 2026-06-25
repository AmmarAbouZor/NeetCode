// Greedy solution based on that we need to look for the smallest values in cards,
// then check if we can form a straight group starting from it then remove the values
// from hand. Then to start over with the min number of remaining cards until there are
// no more cards left.

// Shorter Explanation:
// Always start from the smallest remaining card. That card cannot be placed
// later in another group, so it must start a group of length group_size.

// Use a BTreeMap to keep card counts sorted and repeatedly get the smallest
// remaining value.
//
// Time: O(n log u), where n = hand.len() and u = number of unique cards.
// Space: O(u).

// This is simpler to follow in the interviews.
#[allow(clippy::manual_is_multiple_of)]
pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    if hand.len() % group_size as usize != 0 {
        return false;
    }

    // Alternative to modulo operator
    // if !hand.len().is_multiple_of(group_size as usize) {
    //     return false;
    // }

    let mut counts = std::collections::BTreeMap::new();

    for num in hand {
        counts.entry(num).and_modify(|c| *c += 1).or_insert(1);
    }

    while !counts.is_empty() {
        let first = *counts.keys().next().unwrap();

        for i in 0..group_size {
            let target = first + i;

            let Some(c) = counts.get_mut(&target) else {
                return false;
            };

            *c -= 1;

            if *c == 0 {
                counts.remove(&target);
            }
        }
    }

    true
}
