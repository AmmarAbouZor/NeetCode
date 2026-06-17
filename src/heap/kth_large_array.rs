// Min Binary Heap: Preferred solution for stability
// Time: O(N logK)
// Space O(K)

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;

    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut nums_heap = BinaryHeap::with_capacity(k + 1);

    for num in nums {
        nums_heap.push(Reverse(num));
        if nums_heap.len() > k {
            let _ = nums_heap.pop();
        }
    }

    nums_heap
        .peek()
        .expect("nums is not empty and k <= nums.len()")
        .0
}

// Quick Select: Can provide slightly better performance with risk for much worse
// performance in worse-case scenarios. Also the space is O(1)
// Time: Average O(N), Worst case O(N^2)
// Space: O(1)

pub fn find_kth_largest_quick_select(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut nums = nums;

    // Kth value means k - 1 in zero based indexing system
    let target = k - 1;
    let mut left = 0;
    let mut right = nums.len() - 1;

    loop {
        let pivot = partition(&mut nums, left, right);
        if pivot == target {
            return nums[pivot];
        }

        if pivot > target {
            right = pivot - 1;
        } else {
            left = pivot + 1;
        }
    }
}

/// Partitions the provided slice and its bounds, returning the pivot position
fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
    let pivot_val = nums[right];

    let mut insert = left;
    for idx in left..right {
        let val = nums[idx];
        if val >= pivot_val {
            nums.swap(insert, idx);
            insert += 1;
        }
    }

    nums.swap(insert, right);
    insert
}
