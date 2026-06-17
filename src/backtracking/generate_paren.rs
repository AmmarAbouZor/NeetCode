// Backtracking but without the for loop pattern as we wanted to avoid generate
// invalid parentheses instead of validating them at the end.
// However the concepts are the same. We add something and undo after dfs return.

//  Time: O(C_n * n): Catalan number for valid combination + 2n combination length
//  Output space: O(C_n * n)
//  Extra space: O(n): recursion stack + current string

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let n = n as usize;

    let mut res = Vec::new();

    dfs(n, 0, 0, &mut String::new(), &mut res);

    res
}

fn dfs(n: usize, open: usize, close: usize, curr: &mut String, res: &mut Vec<String>) {
    if curr.len() == n * 2 {
        res.push(curr.clone());
        return;
    }

    if open < n {
        curr.push('(');
        dfs(n, open + 1, close, curr, res);
        curr.pop();
    }

    if close < open {
        curr.push(')');
        dfs(n, open, close + 1, curr, res);
        curr.pop();
    }
}
