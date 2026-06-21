// Also here English lowercase chars is the indicator.
//
// Time O(N * 26) simplified O(N) one loop on the input with two sliding window
// Space O(2 * 26) simplified as O(1)

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let mut target = [0; 26];
    for b in s1.as_bytes() {
        target[(b - b'a') as usize] += 1;
    }

    let mut window = [0; 26];

    let mut left = 0;
    let s2 = s2.as_bytes();
    for right in 0..s2.len() {
        window[(s2[right] - b'a') as usize] += 1;

        if right - left + 1 < s1.len() {
            continue;
        }

        if window == target {
            return true;
        }

        window[(s2[left] - b'a') as usize] -= 1;
        left += 1;
    }

    false
}
