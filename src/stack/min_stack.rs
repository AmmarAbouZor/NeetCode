//! We need to have two stacks here. One for the values and one to the min
//! at the time of adding each value to the stack.
//! This will make each operation O(1) in time but O(N) in space

#[derive(Default)]
pub struct MinStack {
    values: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, val: i32) {
        self.values.push(val);
        let prev_min = self.mins.last().copied().unwrap_or(i32::MAX);
        let min = prev_min.min(val);
        self.mins.push(min);
    }

    pub fn pop(&mut self) {
        let _ = self.values.pop();
        let _ = self.mins.pop();
    }

    pub fn top(&self) -> i32 {
        *self
            .values
            .last()
            .expect("This will be called on valid items only")
    }

    pub fn get_min(&self) -> i32 {
        *self
            .mins
            .last()
            .expect("This will be called on valid items only")
    }
}
