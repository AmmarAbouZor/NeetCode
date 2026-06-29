use std::{cmp::Reverse, collections::BinaryHeap};

// Two heaps:
// * `lower` is a max-heap for the smaller half of numbers.
// * `upper` is a min-heap for the larger half of numbers.
//
// Invariant:
// * Every value in lower <= every value in upper
// * lower.len() either equals upper.len() or bigger by only one.
//
// This lets `find_median` read the answer directly from the heap tops
//
// add_num: O(logN)
// find_median: O(1)
// Space: O(N)

#[derive(Default)]
pub struct MedianFinder {
    lower: BinaryHeap<i32>,
    upper: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, num: i32) {
        let Self { lower, upper } = self;

        if lower.peek().is_none_or(|l_max| num <= *l_max) {
            lower.push(num);
        } else {
            upper.push(Reverse(num));
        }

        // Keep `lower` as the heap with the extra element when the total count is odd.
        if lower.len() > upper.len() + 1 {
            let val = lower.pop().unwrap();
            upper.push(Reverse(val))
        } else if upper.len() > lower.len() {
            let Reverse(val) = upper.pop().unwrap();
            lower.push(val);
        }
    }

    pub fn find_median(&self) -> f64 {
        let Self { lower, upper } = self;

        if lower.len() > upper.len() {
            lower.peek().copied().unwrap() as f64
        } else {
            let max_lower = lower.peek().copied().unwrap() as f64;
            let min_upper = upper.peek().unwrap().0 as f64;

            (max_lower + min_upper) / 2.0
        }
    }
}
