// Code follows how add works in normal arithmetic.
// Start from the right and carry while digits are 9.
// In case of all nines then we insert 1 at start

// Time: O(n)
// Space: extra O(1). With results O(n)
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    for idx in (0..digits.len()).rev() {
        if digits[idx] < 9 {
            digits[idx] += 1;

            return digits;
        }

        digits[idx] = 0;
    }

    digits.insert(0, 1);

    digits
}
