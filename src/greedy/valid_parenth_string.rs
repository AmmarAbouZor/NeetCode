// Greedy range solution.
//
// Instead of choosing what each '*' means immediately, keep a range for how
// many unmatched '(' could exist after reading each prefix of the string.
//
// left_min = the smallest possible number of unmatched '('.
// left_max = the largest possible number of unmatched '('.
//
// For '*', we update both sides of the range:
// - use '*' as ')'     => unmatched '(' can decrease, so left_min -= 1
// - use '*' as '('     => unmatched '(' can increase, so left_max += 1
// - use '*' as empty   => covered by staying somewhere inside the range
//
// If left_max becomes negative, even the best interpretation has too many ')'.
// If left_min becomes negative, clamp it to 0 because we can choose some '*' as
// empty instead of ')' to avoid having fewer than zero unmatched opens.
//
// At the end, the string is valid only if 0 unmatched '(' is still possible.
//
// Time: O(n)
// Space: O(1)
pub fn check_valid_string(s: String) -> bool {
    let mut left_min = 0_isize;
    let mut left_max = 0_isize;

    for b in s.as_bytes() {
        match b {
            b'(' => {
                // A real '(' must increase every possible open-count state.
                left_min += 1;
                left_max += 1;
            }
            b')' => {
                // A real ')' consumes one unmatched '(' from every state.
                left_min -= 1;
                left_max -= 1;
            }
            b'*' => {
                // '*' could be ')' for the minimum, or '(' for the maximum.
                left_min -= 1;
                left_max += 1;
            }
            invalid => panic!("{invalid}"),
        }

        // Even treating every flexible '*' as '(' cannot balance this prefix.
        if left_max < 0 {
            return false;
        }

        // Negative unmatched opens is impossible. Clamp to 0 because a previous
        // '*' can be interpreted as empty instead of ')'.
        left_min = left_min.max(0);
    }

    left_min == 0
}
