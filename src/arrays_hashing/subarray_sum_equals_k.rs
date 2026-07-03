use std::collections::HashMap;

// Prefix sum + frequency map.
//
// If current prefix sum is `prefix_sum`, then a previous prefix `needed` forms
// a subarray ending here with sum k when:
//
// prefix_sum - needed = k
// needed = prefix_sum - k
//
// `pref_count[p]` = how many times prefix sum `p` appeared before.
// Initialize with prefix sum 0 so subarrays starting at index 0 are counted.
//
// Time: O(n)
// Space: O(n)

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut pref_count: HashMap<i32, usize> = HashMap::new();
    pref_count.insert(0, 1);

    let mut prefix_sum = 0;
    let mut counts = 0;

    for num in nums {
        prefix_sum += num;
        let needed = prefix_sum - k;
        if let Some(&c) = pref_count.get(&needed) {
            counts += c;
        }

        pref_count
            .entry(prefix_sum)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    counts as i32
}
