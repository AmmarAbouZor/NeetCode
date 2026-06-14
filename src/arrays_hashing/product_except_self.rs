// O(N) Using prefix and suffix using the same result to store them without 
// Storing them in extra vectors.
// This will also avoid using division.
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![1; nums.len()];

    let mut prefix = 1;
    for idx in 0..nums.len() {
        res[idx] = prefix;
        prefix *= nums[idx];
    }

    let mut suffix = 1;
    for idx in (0..nums.len()).rev() {
        res[idx] *= suffix;
        suffix *= nums[idx];
    }

    res
}


// O(N) Using prefix and suffix in case divide isn't allowed
// This will allocate two vectors for prefix and suffix.
pub fn product_except_self_extra_vecs(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix = Vec::with_capacity(nums.len());

    let mut product = 1;
    for &num in &nums {
        product *= num;
        prefix.push(product);
    }

    let mut suffix = vec![0; nums.len()];
    product = 1;
    for (idx, &num) in nums.iter().enumerate().rev() {
        product *= num;
        suffix[idx] = product;
    }

    let mut res = Vec::with_capacity(nums.len());

    for idx in 0..nums.len() {
        let pre = idx.checked_sub(1).map(|i| prefix[i]).unwrap_or(1);
        let suff = suffix.get(idx + 1).copied().unwrap_or(1);

        res.push(pre * suff);
    }

    res
}


// O(N) with division. It's simple but it may be not enough for the interview.
pub fn product_except_self_with_division(nums: Vec<i32>) -> Vec<i32> {
    let mut product_no_zero = 1;
    let mut zeros_idx = Vec::new();

    for (idx, &num) in nums.iter().enumerate() {
        if num == 0 {
            zeros_idx.push(idx);
        } else {
            product_no_zero *= num;
        }
    }

    // Cases we have:
    // * No zero: Devide product on each number
    // * One zero only: In that case all items will be zero except for zero item itself.
    // * Many zeros: All are zeros regardless.
    match zeros_idx.len() {
        0 => nums.into_iter().map(|n| product_no_zero / n).collect(),
        1 => {
            let mut res = vec![0; nums.len()];
            let idx = zeros_idx[0];
            res[idx] = product_no_zero;
            res
        }
        _ => vec![0; nums.len()],
    }
}
