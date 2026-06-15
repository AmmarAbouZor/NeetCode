// Time: O(N) because each day is pushed and popped from the monotonic stack at most once.
// Space: O(N) due to the result array and the stack
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];

    // Stack for the indices for unresolved days.
    let mut unresolved = Vec::<usize>::new();

    for (idx, &temp) in temperatures.iter().enumerate() {
        while let Some(&prev_idx) = unresolved.last() {
            if temperatures[prev_idx] >= temp {
                break;
            }
            let _ = unresolved.pop();

            res[prev_idx] = (idx - prev_idx) as i32;
        }

        unresolved.push(idx);
    }

    res
}
