// Solution with two pointers:
// We start on each side and move the pointer with less height.

// Time: O(n): Two pointers running in linear time
// Space: O(1)

pub fn max_area(heights: Vec<i32>) -> i32 {
    let len = heights.len();
    if len < 2 {
        return 0;
    }

    let mut l = 0;
    let mut r = len - 1;

    let mut area: i32 = 0;
    while l < r {
        let height = heights[l].min(heights[r]);
        let width = r as i32 - l as i32;
        area = area.max(height * width);

        if heights[l] <= heights[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    area
}
