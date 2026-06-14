/// Function signature as in NeetCode
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 {
        return Vec::new();
    }

    use std::collections::HashMap;

    // Complement map
    let mut compl_map = HashMap::<i32, i32>::new();

    for (idx, num) in nums.iter().enumerate() {
        if let Some(first_idx) = compl_map.get(num) {
            return vec![*first_idx, idx as i32];
        }

        // Complement
        let comp = target - num;

        compl_map.insert(comp, idx as i32);
    }

    Vec::new()
}

/// This function if I can choose the signature of it
pub fn two_sum_my(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    if nums.len() < 2 {
        return None;
    }

    use std::collections::HashMap;

    // Complement map
    let mut compl_map = HashMap::<i32, usize>::new();

    for (idx, num) in nums.iter().enumerate() {
        if let Some(first_idx) = compl_map.get(num) {
            return Some((*first_idx, idx));
        }

        // Complement
        let comp = target - num;

        compl_map.insert(comp, idx);
    }

    None
}
