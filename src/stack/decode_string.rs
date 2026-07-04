// Stack simulation for nested encoded strings.
//
// `curr_txt` is the string being built at the current nesting level.
// `curr_num` is the repeat count being parsed before a `[`.
//
// When we see `[`, we are entering a nested level:
// - save the current context `(curr_txt, curr_num)` on the stack
// - reset `curr_txt` and `curr_num` for the inner string
//
// When we see `]`, the current inner string is complete:
// - pop the previous context
// - append the current string `count` times to the previous string
// - make that combined string the current string again
//
// Digits may have multiple characters, so build the number with:
// curr_num = curr_num * 10 + digit
//
// Time: O(output length), because repeated strings must be written.
// Space: O(output length + nesting depth).

// Key mental model:
// When we enter a bracket, save the current context.
// When we leave a bracket, expand the completed inner string into that saved context.

pub fn decode_string(s: String) -> String {
    let mut stack: Vec<(String, u32)> = Vec::new();

    let mut curr_txt = String::new();
    let mut curr_num = 0;

    for ch in s.chars() {
        match ch {
            '[' => {
                // Enter a nested block; save the outer context.
                stack.push((curr_txt, curr_num));
                curr_txt = String::new();
                curr_num = 0;
            }
            ']' => {
                // Finish current block and expand it into the previous context.
                let (mut prev, count) = stack.pop().unwrap();

                for _ in 0..count {
                    prev.push_str(&curr_txt);
                }

                curr_txt = prev;
            }
            ch => {
                if let Some(digit) = ch.to_digit(10) {
                    // Counts can have multiple digits
                    curr_num = curr_num * 10 + digit;
                } else {
                    curr_txt.push(ch);
                }
            }
        }
    }

    curr_txt
}
