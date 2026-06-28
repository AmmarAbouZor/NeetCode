// We need the middle value(s) of the two arrays as if they were merged.
//
// A linear merge would move two pointers and repeatedly take the smaller value
// until reaching the middle, which costs O(n + m). Binary search does the same
// reasoning faster by finding the correct split point instead of building or
// scanning the merged array.
//
// Binary search over the partition point of the shorter sorted array.
//
// Split both arrays into left and right halves. The left half must contain
// `left_len = total.div_ceil(2)` elements, so choosing `short_cut` determines
// `long_cut = left_len - short_cut`.
//
// A valid partition has:
// short_left <= long_right && long_left <= short_right.
//
// Then all values on the left are <= all values on the right. For add total length
// the median is max(left side). For even total length, it is the average of
// max(left side) and min(right side).
//
// If partition isn't valid then we move either low or high and reapply.
//
// Time: O(log(min(n, m))). As we pick the smaller vector for binary search.
// Space: O(1)

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // We pick the shorter array to do binary search on it.
    let (short, long) = if nums1.len() <= nums2.len() {
        (&nums1, &nums2)
    } else {
        (&nums2, &nums1)
    };

    let total = short.len() + long.len();
    // Left side gets the extra element when total is odd.
    let left_len = total.div_ceil(2);

    let mut low = 0;
    let mut high = short.len();

    while low <= high {
        let short_cut = (low + high) / 2;
        let long_cut = left_len - short_cut;

        let short_left = if short_cut == 0 {
            i32::MIN
        } else {
            short[short_cut - 1]
        };

        let short_right = if short_cut == short.len() {
            i32::MAX
        } else {
            short[short_cut]
        };

        let long_left = if long_cut == 0 {
            i32::MIN
        } else {
            long[long_cut - 1]
        };

        let long_right = if long_cut == long.len() {
            i32::MAX
        } else {
            long[long_cut]
        };

        if short_left <= long_right && long_left <= short_right {
            if total % 2 == 1 {
                return short_left.max(long_left) as f64;
            }

            let left_max = short_left.max(long_left) as f64;
            let right_min = short_right.min(long_right) as f64;

            return (left_max + right_min) / 2.0;
        }

        if short_left > long_right {
            high = short_cut - 1;
        } else {
            low = short_cut + 1;
        }
    }

    panic!("Unreachable!");
}
