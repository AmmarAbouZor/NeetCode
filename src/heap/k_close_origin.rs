use std::collections::BinaryHeap;

// Time: O(N LogK) as we are iterating through all points then applying insert and pop
//		on the binary heap.
// Space: O(K + 1) ~= O(K) for binary heap

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;

    let mut heap = BinaryHeap::with_capacity(k + 1);

    for p in points {
        let x = p[0];
        let y = p[1];
        // Sqrt returns f32 , which doesn't implement Ord and can't be used
        // in binary heap. We'll use the square dist instead since it's more
        // accurate then `isqrt()` function.
        // An alternative is to define our wrapper over f32 and implement Ord for
        // it manually using a specific delta.
        let dist_square = x.pow(2) + y.pow(2);

        heap.push((dist_square, x, y));

        if heap.len() > k {
            let _ = heap.pop();
        }
    }

    heap.into_iter().map(|(_dist, x, y)| vec![x, y]).collect()
}
