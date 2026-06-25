// Greedy solution based on two actions:
// * Triplets with any value greater than the matching target should be skipped
//   because merge uses max, so an oversized value can never be reduced.
// * Among valid triplets, each target position must be matched at least once.

// Note: It's possible to check if all matches are true from inside the loop and return early.
// I'll do that in the interview but for now keep it like this for clarity.

// Time: O(N): We are going through triplets once
// Space: O(1)

pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let mut matches = [false; 3];

    for tri in triplets {
        let valid = (0..3).all(|i| tri[i] <= target[i]);
        if !valid {
            continue;
        }

        for idx in 0..3 {
            matches[idx] |= tri[idx] == target[idx];
        }
    }

    matches.iter().all(|m| *m)
}
