use std::collections::HashMap;

// Dynamic programming over index and reachable sums.
// Sums can be negative, so a HashMap is simpler than indexing a Vec by sum.

// DP State:
// dp[i][sum] = number of ways to reach sum
//              using the first i numbers.

// Optimized solution is simpler to understand and write but I've provided the full
// one below as reading both of them is better to understand.

// Time: O(n * s), where s is the number of reachable sums.
// In the worst case, s <= 2 * sum(nums) + 1.
// Space: O(s) for the optimized version.
// Full version uses O(n * s).

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = HashMap::new();
    // There is one way to get sum of zero from zero items.
    dp.insert(0, 1);

    for num in nums {
        let mut next_dp = HashMap::new();

        for (sum, count) in dp {
            *next_dp.entry(sum + num).or_insert(0) += count;
            *next_dp.entry(sum - num).or_insert(0) += count;
        }

        dp = next_dp;
    }

    dp.get(&target).copied().unwrap_or_default()
}

// Same solution using full table Vec<HashMap<i32, i32>>
pub fn find_target_sum_ways_full(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut dp = vec![HashMap::new(); n + 1];
    dp[0].insert(0, 1);

    for (idx, num) in nums.into_iter().enumerate() {
        // We need a way to tell the borrow checker that we are changing
        // the current row only and won't touch the previous ones.
        // split_at_mut proves prev and current rows are non-overlapping.
        let (prev_rows, cur_rows) = dp.split_at_mut(idx + 1);
        let prev = &prev_rows[idx];
        let cur = &mut cur_rows[0];
        for (&sum, &count) in prev.iter() {
            *cur.entry(sum + num).or_insert(0) += count;
            *cur.entry(sum - num).or_insert(0) += count;
        }
    }

    dp[n].get(&target).copied().unwrap_or_default()
}
