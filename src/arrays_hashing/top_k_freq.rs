/// O(N): Optimal solutions with bucket sort instead of heap
pub fn top_k_frequent_bucket_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = std::collections::HashMap::new();
    let n = nums.len();

    // First O(N)
    nums.into_iter().for_each(|n| {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
    });

    let mut buckets = vec![Vec::new(); n + 1];

    // Second O(N)
    for (num, counts) in counts {
        buckets[counts].push(num);
    }

    let k = k as usize;
    let mut ans = Vec::with_capacity(k);

    // Third O(N)
    for bucket in buckets.into_iter().rev() {
        for num in bucket {
            ans.push(num);
            if ans.len() == k {
                return ans;
            }
        }
    }

    ans
}

/// O(N log N): Saving space with min_heap but still not enough
/// O(N) iterating and buildign the map , O(Logn) Heap cost
pub fn top_k_frequent_min_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = std::collections::HashMap::new();

    nums.into_iter().for_each(|n| {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
    });

    let mut min_heap = std::collections::BinaryHeap::with_capacity(k as usize + 1);

    for (num, counts) in counts {
        use std::cmp::Reverse;
        min_heap.push((Reverse(counts), num));
        if min_heap.len() > k as usize {
            _ = min_heap.pop();
        }
    }

    min_heap.into_iter().map(|(_counts, num)| num).collect()
}

/// O(N + M log M):
/// O(N) iterating and buildign the map , O(M) heapify, LogM: build answer
/// Complete Heapify wasn't needed. Instead use min heap or use buckets
pub fn top_k_frequent_max_heap(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
    let mut counts = std::collections::HashMap::new();

    nums.into_iter().for_each(|n| {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
    });

    let counts: Vec<_> = counts
        .into_iter()
        .map(|(num, count)| (count, num))
        .collect();

    let mut heap = std::collections::BinaryHeap::from(counts);

    let mut ans = Vec::with_capacity(k as usize);

    while k > 0
        && let Some((_count, num)) = heap.pop()
    {
        ans.push(num);
        k -= 1;
    }

    ans
}
