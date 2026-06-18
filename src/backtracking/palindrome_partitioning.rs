// Backtracking following the for loop pattern.
// The results types follows the algorithm more than real word case as this
// will be useful as Vec<String> and not Vec<Vec<String>>.

// Time: O(2^n * n^2): We branch on each node + Check is palindrome + Clone the results
// Extra space: O(n), excluding output: For recursion stack & Storing partition
// Space with results O(2^n * n)

pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    // Input is valid ascii
    let s = s.as_bytes();

    dfs(s, 0, &mut Vec::new(), &mut res);

    res
}

fn dfs(s: &[u8], idx: usize, cur: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
    if idx >= s.len() {
        res.push(cur.clone());
        return;
    }

    for j in idx..s.len() {
        if !is_palindrome(s, idx, j) {
            continue;
        }
        let txt = String::from_utf8_lossy(&s[idx..=j]).into_owned();
        cur.push(txt);
        dfs(s, j + 1, cur, res);
        cur.pop();
    }
}

fn is_palindrome(s: &[u8], mut l: usize, mut r: usize) -> bool {
    while l < r {
        if s[l] != s[r] {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}
