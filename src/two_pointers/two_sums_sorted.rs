// O(N) time , O(1) space
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let l = numbers[left];
        let r = numbers[right];
        let sum = l + r;

        use std::cmp::Ordering;

        match sum.cmp(&target) {
            Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
        }
    }

    panic!("We must have solution")
}
