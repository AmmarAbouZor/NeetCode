// O(N) in time, O(1) in space

pub fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let bytes = s.as_bytes();

    let mut left = 0;
    let mut right = bytes.len() - 1;

    while left < right {
        let l = bytes[left];

        if !l.is_ascii_alphanumeric() {
            left += 1;
            continue;
        }

        let r = bytes[right];
        if !r.is_ascii_alphanumeric() {
            right -= 1;
            continue;
        }

        if !l.eq_ignore_ascii_case(&r) {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}
