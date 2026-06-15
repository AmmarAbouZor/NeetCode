use std::collections::HashMap;

// Time:
// new(): O(1)
// set(): average/amortized O(1): Insert to HashMap + push to vector
// get(): O(log m), where m is values stored for that key (Binary search)

// Space is O(N * m) where n: is the number of key, values pair and m is the maximum number of
// of values associated with one key.

// Current solution is built based on that: Timestamps are inserted in increasing order per key

// NOTE: check partition_point() implementation below.

#[derive(Default)]
pub struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        let Some(values) = self.map.get(&key) else {
            return String::default();
        };

        // There is no remove function so we know that we don't have empty vectors.
        let mut left = 0;
        let mut right = values.len();
        // NOTE: check partition_point() implementation below

        let mut ans = String::default();

        while left < right {
            let mid = left + (right - left) / 2;

            // The condition here is different than normal binary trees.
            // I didn't get it right on my own even though it's so simple.
            if values[mid].0 <= timestamp {
                ans = values[mid].1.to_owned();
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        ans
    }
}

// Using partition_point to avoid the binary_tree implementation.
// fn get(&self, key: String, timestamp: i32) -> String {
//     let Some(values) = self.map.get(&key) else {
//         return String::default();
//     };
//
//     let idx = values.partition_point(|(ts, _)| *ts <= timestamp);
//
//     if idx == 0 {
//         String::default()
//     } else {
//         values[idx - 1].1.clone()
//     }
// }
