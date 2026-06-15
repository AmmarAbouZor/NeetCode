// O(N) in time and space
pub fn is_valid(s: String) -> bool {
    let mut open_stack = Vec::new();

    for ch in s.chars() {
        if is_open_pare(ch) {
            open_stack.push(ch);
        } else {
            // We will assume that we have valid input only.
            // Making this a close parentheses.

            let Some(open_pare) = open_stack.pop() else {
                return false;
            };
            if !is_valid_pair(open_pare, ch) {
                return false;
            }
        }
    }

    open_stack.is_empty()
}

fn is_open_pare(ch: char) -> bool {
    matches!(ch, '(' | '[' | '{')
}

fn is_valid_pair(l: char, r: char) -> bool {
    matches!((l, r), ('(', ')') | ('[', ']') | ('{', '}'))
}

#[test]
fn test_open_pare() {
    assert!(is_open_pare('('));
    assert!(!is_open_pare(']'));
}
