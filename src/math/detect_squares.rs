use std::collections::HashMap;

// Store counts for each point because duplicate points are allowed.
//
// For count(point), treat every stored point with equal non-zero dx/dy as the
// opposite diagonal corner. The other two corners must exist to form a square.
//
// add: O(1) average
// count: O(p), where p is the number of unique points
// space: O(p)
#[derive(Default)]
pub struct CountSquares {
    points: HashMap<(i32, i32), usize>,
}

impl CountSquares {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, point: Vec<i32>) {
        let x = point[0];
        let y = point[1];

        self.points
            .entry((x, y))
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let tar_x = point[0];
        let tar_y = point[1];

        let mut count = 0;

        for (&(px, py), &diag_count) in self.points.iter() {
            let dx = (px - tar_x).abs();
            let dy = (py - tar_y).abs();

            // A square needs an opposite diagonal corner with equal side lengths.
            if dx != dy || dx == 0 {
                continue;
            }

            let Some(&corner1) = self.points.get(&(tar_x, py)) else {
                continue;
            };

            let Some(&corner2) = self.points.get(&(px, tar_y)) else {
                continue;
            };

            // Multiplication handles duplicate points at each required corner.
            count += diag_count * corner1 * corner2;
        }

        count as i32
    }
}
