// Binary addition relies on two operations:
// a ^ b = sum the number without carry
// (a & b) << 1 = carry bits shifted to the next position

// Time: O(32) ~= O(1)
// Space: O(1)

// This is longer version for more understand
pub fn get_sum_long(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let sum_no_carry = a ^ b;
        let carry = (a & b) << 1;
        a = sum_no_carry;
        b = carry;
    }

    a
}

// Shorter version with one variable less
pub fn get_sum_short(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let carry = (a & b) << 1;
        a ^= b;
        b = carry;
    }

    a
}
