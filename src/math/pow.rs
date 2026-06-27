// Solution using fast exponentiation.
//
// Instead of multiplying x by itself n times, use the binary representation of n.
// Each bit tells us whether the current base contributes to the final result.
//
// Example: x^13
// 13 in binary is 1101, so:
// x^13 = x^8 * x^4 * x^1
//
// The loop keeps:
// - result: product of powers we decided to include
// - base: current power of x, starting at x, then x^2, x^4, x^8, ...
// - power: remaining exponent bits to process
//
// On each iteration:
// - if power is odd, the lowest bit is 1, so include current base in result
// - square base to move to the next power of two
// - divide power by 2 to shift to the next bit
//
// Negative exponents are handled by using 1 / x as the base and making power positive.
// Convert n to i64 first so negating i32::MIN is safe.
//
// Time: O(log n)
// Space: O(1)
pub fn my_pow(x: f64, n: i32) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    if n == 0 {
        return 1.0;
    }

    let mut power = n as i64;
    let mut base = x;

    if power < 0 {
        base = 1.0 / base;
        power = -power;
    }

    let mut result = 1.0;

    while power > 0 {
        if power % 2 == 1 {
            result *= base;
        }

        base *= base;
        power /= 2;
    }

    result
}

// Brute force solution
// Time: O(n)
// Space: O(1)
pub fn my_pow_brute(x: f64, n: i32) -> f64 {
    if x == 0.0 {
        return 0.0;
    }

    if n == 0 {
        return 1.0;
    }

    let mut res = 1.0;

    for _ in 0..n.abs() {
        res *= x;
    }

    if n > 0 { res } else { 1.0 / res }
}
