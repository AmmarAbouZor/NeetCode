pub fn contains_duplicates(nums: &[i32]) -> bool {
    use std::collections::HashSet;

    let mut nums_set = HashSet::new();

    for num in nums {
        // HashSet will return the stored value if we insert a value
        // which is already in the set.
        if !nums_set.insert(num) {
            return true;
        }
    }

    false
}

pub fn contains_duplicates_idiomatic(nums: &[i32]) -> bool {
    use std::collections::HashSet;

    let mut nums_set = HashSet::new();

    nums.iter().any(|num| !nums_set.insert(*num)) 
}
