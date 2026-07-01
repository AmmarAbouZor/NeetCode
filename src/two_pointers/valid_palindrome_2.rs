// Two pointers with one allowed deletion.
//
// Move inward while chars match. At the first mismatch, the only two possible
// fixes are deleting the left char or deleting the right char. Check whether
// either remaining slice is a palindrome.
//
// Time: O(n)
// Space: O(1)

pub fn valid_palindrome(s: String) -> bool {
    let s = s.as_bytes();

    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if s[left] == s[right] {
            left += 1;
            right -= 1;
        } else {
            return valid_slice(s, left + 1, right) || valid_slice(s, left, right - 1);
        }
    }

    true
}

// Checks whether s[left..=right] is a palindrome without further deletion.
fn valid_slice(s: &[u8], mut left: usize, mut right: usize) -> bool {
    while left < right {
        if s[left] != s[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}
