// Backtracking without loop pattern.

// Time: O(4^n * n): Branching with max 4 options + Clone on each branch
// Space exclude output: O(N): Recursion Stack + Clone
// Space O(4^n * n)

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let digits = digits.as_bytes();
    let mut res = Vec::new();
    dfs(digits, 0, &mut String::new(), &mut res);

    res
}

fn dfs(digits: &[u8], idx: usize, cur: &mut String, res: &mut Vec<String>) {
    if idx == digits.len() {
        res.push(cur.clone());
        return;
    }

    for &ch in digit_chars(digits[idx]) {
        cur.push(ch);
        dfs(digits, idx + 1, cur, res);
        cur.pop();
    }
}

fn digit_chars(digit_byte: u8) -> &'static [char] {
    match digit_byte {
        b'2' => &['a', 'b', 'c'],
        b'3' => &['d', 'e', 'f'],
        b'4' => &['g', 'h', 'i'],
        b'5' => &['j', 'k', 'l'],
        b'6' => &['m', 'n', 'o'],
        b'7' => &['p', 'q', 'r', 's'],
        b'8' => &['t', 'u', 'v'],
        b'9' => &['w', 'x', 'y', 'z'],
        invalid => panic!("invalid input {invalid}"),
    }
}
