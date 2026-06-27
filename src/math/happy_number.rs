// Typical solution is to use a HashSet but it's possible to avoid allocating space
// using Floyd fast and slow pointers making space O(1)

// Time: O(logN)
// Space: O(1)
pub fn is_happy_fast_slow(n: i32) -> bool {
    let mut slow = n;
    let mut fast = calc_squares(n);

    loop {
        fast = calc_squares(fast);
        fast = calc_squares(fast);
        if fast == 1 {
            return true;
        }
        slow = calc_squares(slow);
        if slow == fast {
            return false;
        }
    }
}

// Solution to detect duplicates using HashSet
// Time: O(logN)
// Space: O(logN)
pub fn is_happy_set(n: i32) -> bool {
    let mut seen = std::collections::HashSet::new();

    let mut n = n;
    seen.insert(n);
    loop {
        n = calc_squares(n);
        if n == 1 {
            return true;
        }

        if !seen.insert(n) {
            return false;
        }
    }
}

fn calc_squares(mut n: i32) -> i32 {
    let mut res = 0;

    while n != 0 {
        res += (n % 10).pow(2);
        n /= 10;
    }

    res
}
