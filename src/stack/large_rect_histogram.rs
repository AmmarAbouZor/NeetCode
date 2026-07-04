// Monotonic stack keeping bars which can extend forward and calculating the area of popped
// bars from top of stack, keeping track on max area.

// To calculate the area we need to get the width as bars can still extend to left while forming
// the area. The width can be calculated from the stack and current index as:
// width = if Some(left) = stack.last() { cur_idx - left - 1 } else { cur_idx }

// To ensure all bars in stack are processed we add a sentinel 0 to the end of heights.

// Time: O(n) as we are visiting each item once in iteration and once in stack
// Space: O(n) for stack.

// More idiomatic rust solution using pop_if() and iter::chain + iter::once
pub fn largest_rectangle_area_idiomatic(heights: Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut largest = 0;

    use std::iter;
    for (idx, cur_h) in heights.iter().copied().chain(iter::once(0)).enumerate() {
        while let Some(top) = stack.pop_if(|t| cur_h < heights[*t]) {
            // `idx` is the first shorter bar on the right.
            // `left` is the nearest shorter bar on the left.
            // The rectangle can only use bars between them: left + 1..idx - 1.
            let width = if let Some(&left) = stack.last() {
                idx - left - 1
            } else {
                idx
            };

            let area = width as i32 * heights[top];
            largest = largest.max(area);
        }

        stack.push(idx);
    }

    largest
}

// Variation of same function using more simple code without rust fancy stuff.
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut heights = heights;

    // Add zero to force processing all bars in histogram
    heights.push(0);

    let mut stack = Vec::new();

    let mut largest = 0;

    for (idx, &cur_h) in heights.iter().enumerate() {
        while let Some(&top) = stack.last() {
            let top_h = heights[top];
            if cur_h >= top_h {
                break;
            }

            stack.pop();

            let width = if let Some(&left) = stack.last() {
                idx - left - 1
            } else {
                idx
            };

            largest = largest.max(width as i32 * top_h);
        }

        stack.push(idx);
    }

    largest
}
