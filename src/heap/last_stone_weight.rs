// Time: O(NlogN): O(N) initial heapify + O(2 * NLogN) smashing loop having 2 as worst case
// 	     as each smash is O(LogN)
// Space: O(N)

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut stones = std::collections::BinaryHeap::from(stones);

    while stones.len() > 1 {
        let s1 = stones.pop().expect("Heap len is greater than 1");
        let s2 = stones.pop().expect("Heap len is greater than 1");
        let diff = s1 - s2;

        if diff != 0 {
            stones.push(diff);
        }
    }

    stones.pop().unwrap_or_default()
}
