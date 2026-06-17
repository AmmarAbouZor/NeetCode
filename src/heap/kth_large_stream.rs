use std::{cmp::Reverse, collections::BinaryHeap};

// Use min-heap as it stores the current k largest values seen so far. Its root is
// the smallest among those k, which is the kth largest overall.
// Min-heap to avoid keeping values that aren't needed in the heap.

// Time O(m * logk) as m is num of calls made to add()
// Space O(k)
// Object storage: O(k)

pub struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    // Time: O(N logK) where n is nums.len()
    // Space: O(k)
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        // Input guarantee
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for num in nums {
            heap.push(Reverse(num));
            if heap.len() > k {
                let _ = heap.pop();
            }
        }

        Self { heap, k }
    }

    // Time: O(logK)
    // Space: O(1)
    pub fn add(&mut self, val: i32) -> i32 {
        let Self { heap, k } = self;

        heap.push(Reverse(val));
        if heap.len() > *k {
            let _ = heap.pop();
        }

        heap.peek().unwrap().0
    }
}

// This approach will use a max-heap which will work but it will keep all items in
// the heap even that we don't need them. A better approach is a min-heap here.
pub struct KthLargestMaxHeap {
    heap: BinaryHeap<i32>,
    k: usize,
    cache: Vec<i32>,
}

impl KthLargestMaxHeap {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        // Input guarantee
        let k = k as usize;
        let heap = BinaryHeap::from(nums);
        let cache = Vec::new();

        Self { heap, k, cache }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        let Self { heap, k, cache } = self;

        debug_assert!(cache.is_empty());

        heap.push(val);

        for _ in 0..*k - 1 {
            // Input guarantee
            let item = heap.pop().unwrap();
            cache.push(item);
        }

        let res = heap.peek().copied().unwrap();

        heap.extend(cache.drain(..));

        res
    }
}
