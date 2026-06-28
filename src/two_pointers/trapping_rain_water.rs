// Two-pointer solution with running max walls.
//
// Water above a bar is limited by a shorter side.
//
// water[i] = min(max_left, max_right) - height[i]
//
// Instead of pre-computing max-left and max-right array, keep one pointer at
// each end and track the best wall seen from each side.
//
// If height[left] < height[right], the right side already has a wall tall enough
// to trap water at `left`, so the amount depends only on `max_left`.
// Otherwise, process `right` using `max_right`.
//
// Update the side max before adding water so the added amount is never negative.
//
// Time: O(n)
// Space: O(1)
pub fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_left = 0;
    let mut max_right = 0;

    let mut water = 0;

    while left < right {
        if height[left] < height[right] {
            // Right boundary is tall enough => left side decides the water here.
            max_left = max_left.max(height[left]);
            water += max_left - height[left];
            left += 1;
        } else {
            // Left boundary is tall enough => right side decides the water here.
            max_right = max_right.max(height[right]);
            water += max_right - height[right];
            right -= 1;
        }
    }

    water
}
