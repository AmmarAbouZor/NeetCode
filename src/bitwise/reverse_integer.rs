// Take each digit with modulo operator shifting the input by dividing by 10.
// Then update the answer with ans = prev_ans * 10 + current_digit.
// It's important to consider overflow scenarios.

// Rust keeps the sign on `%`, so negative numbers work with the same logic.

// Time & Space: O(1)

pub fn reverse(x: i32) -> i32 {
    let mut x = x;

    let mut res = 0_i32;

    while x != 0 {
        let digit = x % 10;
        x /= 10;

        res = match res.checked_mul(10).and_then(|r| r.checked_add(digit)) {
            Some(num) => num,
            None => return 0,
        };
    }

    res
}
