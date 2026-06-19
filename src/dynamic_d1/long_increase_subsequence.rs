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
