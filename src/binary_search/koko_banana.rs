// Basically we need to find the first k value in range 1..max_k which is max pile length
// To search in this ordered list we can apply binary search instead of the linear
//
// O(nlogm) where n is the input size, m is the maximum value in the array
// * Getting max is O(n)
// * Binary search is O(logm) and can_finish is O(n) making the loop O(nlogm).
//
// Space is O(1) as we didn't need to allocate in the solution.

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    // Define the upper bound for the binary search then apply a binary search between lowest
    // limit 1 and the upper limit to find the minimum value.
    let top_k: i32 = piles.iter().copied().max().expect("Piles will have values");

    let mut l = 1;
    let mut r = top_k + 1;

    while l < r {
        let mid = l + (r - l) / 2;
        if can_finish(&piles, h, mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    l
}

// Check if all piles can be eaten in the given time based on the k value.
fn can_finish(piles: &[i32], h: i32, k: i32) -> bool {
    let mut hours = 0;

    for pile in piles {
        // This is equivalent to ceil(pile , k). Which means:
        // How many full k-sized chunks do we need to cover pile,
        // counting a leftover partial chunk as one more hour?
        // Rust have pile.div_ceil(k) but it unstable.
        hours += (pile + k - 1) / k;
        if hours > h {
            return false;
        }
    }

    true
}
