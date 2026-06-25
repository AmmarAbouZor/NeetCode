// Kadane algorithm principle, but it's more complex here than sum because of negative numbers.
// To handle that, keep track of both the current minimum and maximum product because
// a negative number can turn the smallest product into the largest product.
// We have two similar solutions:
// * One will just swap min and max on negative numbers
// * One will compute the three candidates for each number and pick min and max from them
// Note: Zero edge case is handled in both of them without extra code.

// Time: O(n) we iterate through nums once.
// Space: O(1)

pub fn max_product_swap(nums: Vec<i32>) -> i32 {
    let mut nums = nums.into_iter();
    let Some(first) = nums.next() else {
        return i32::MIN;
    };

    let mut best = first;
    let mut cur_min = first;
    let mut cur_max = first;

    for num in nums {
        if num < 0 {
            std::mem::swap(&mut cur_min, &mut cur_max);
        }

        cur_min = num.min(cur_min * num);
        cur_max = num.max(cur_max * num);

        best = best.max(cur_max);
    }

    best
}

pub fn max_product_candidates(nums: Vec<i32>) -> i32 {
    let mut nums = nums.into_iter();
    let Some(first) = nums.next() else {
        return i32::MIN;
    };

    let mut best = first;
    let mut cur_min = first;
    let mut cur_max = first;

    for num in nums {
        let candidates = [num * cur_max, num * cur_min, num];
        cur_max = candidates
            .into_iter()
            .max()
            .expect("candidates is not empty");
        cur_min = candidates
            .into_iter()
            .min()
            .expect("candidates is not empty");
        best = best.max(cur_max);
    }

    best
}
