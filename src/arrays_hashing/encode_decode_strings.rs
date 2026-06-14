//! NOTE:
//! * Important to remember by_ref when we want to use take() with iterators
//! * Remember to use is_ascii_digit instead of is_digit(10)

pub fn encode(strs: Vec<String>) -> String {
    let mut res = String::new();

    use std::fmt::Write as _;
    for s in strs {
        let count = s.len();
        write!(&mut res, "{count}#{s}").expect("writing to string should never fail");
    }

    res
}

/// Implementaion using chars() and more idiomatic.
pub fn decode(s: String) -> Vec<String> {
    let mut res = Vec::new();

    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        let mut len = ch.to_digit(10).expect("Valid input must start with digit") as usize;

        for ch in chars.by_ref() {
            if let Some(digit) = ch.to_digit(10) {
                len = len * 10 + digit as usize;
            } else {
                assert_eq!(ch, '#');
                break;
            }
        }

        let word: String = chars.by_ref().take(len).collect();

        res.push(word);
    }

    res
}

/// Same implemenation but with index instead of chars and with bytes.
pub fn decode_index(s: String) -> Vec<String> {
    let bytes = s.as_bytes();
    let mut res = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let mut len = 0;

        while bytes[i] != b'#' {
            len = len * 10 + (bytes[i] - b'0') as usize;
            i += 1;
        }

        i += 1; // skip '#'

        let word = s[i..i + len].to_string();
        res.push(word);

        i += len;
    }

    res
}
