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

// This will be quick select solution which averages on O(N) but the worst case is O(N^2)
// This can be a follow up question.
pub fn k_closest_quick_select(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut points = points;

    if k == 0 {
        return Vec::new();
    }

    let mut l = 0;
    let mut r = points.len() - 1;
    // Quickselect targets an index, not a count. The first k closest points occupy
    // indices 0..k, so the kth closest point is at index k - 1.
    let target = k - 1;

    loop {
        let pivot = partition(&mut points, l, r);
        if pivot == target {
            break;
        }

        if pivot > target {
            r = pivot - 1;
        } else {
            l = pivot + 1;
        }
    }

    points.truncate(k);
    points
}

fn partition(points: &mut [Vec<i32>], l: usize, r: usize) -> usize {
    let pivot_dist = points[r][0].pow(2) + points[r][1].pow(2);

    let mut insert = l;
    for idx in l..r {
        let dist = points[idx][0].pow(2) + points[idx][1].pow(2);
        if dist <= pivot_dist {
            points.swap(insert, idx);
            insert += 1;
        }
    }

    points.swap(insert, r);
    insert
}
