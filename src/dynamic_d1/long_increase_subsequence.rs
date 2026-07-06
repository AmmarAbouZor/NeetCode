// Prefix-style bottom-up DP.
//
// lis[i] = length of the longest increasing subsequence ending at index i.
//
// For each i, check all previous indices j < i.
// If nums[j] < nums[i], then nums[i] can extend the subsequence ending at j:
//
// lis[i] = max(lis[i], lis[j] + 1)
//
// The answer is max(lis), because the LIS can end at any index.
//
// Time: O(n^2)
// Space: O(n)

pub fn length_of_lis_suffix(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    // Base: Each element can be it's own subsequence.
    let mut lis = vec![1; n];

    for i in 0..n {
        for j in 0..i {
            if nums[j] < nums[i] {
                lis[i] = lis[i].max(lis[j] + 1);
            }
        }
    }

    lis.into_iter().max().unwrap_or_default()
}

// Bottom-up DP.
// lis[i] is the length of the longest increasing subsequence starting at index i.
// Iterate from right to left so lis[j] is already known for every j > i.
// For each later larger value nums[j], we can extend that subsequence with nums[i].
//
// The answer is max(lis), because the LIS can start at any index.
//
// Time: O(n^2)
// Space: O(n)

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut lis = vec![0; n];
    for i in (0..n).rev() {
        let mut best = 1;
        for j in i + 1..n {
            if nums[i] < nums[j] {
                best = best.max(lis[j] + 1);
            }
        }

        lis[i] = best;
    }

    lis.into_iter().max().unwrap_or_default()
}
