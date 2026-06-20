// Greedy reset pattern.
//
// First check total gas vs total cost. If total gas is smaller, no starting
// station can finish the circuit.
//
// If total gas is enough, a valid start exists. We scan from left to right and
// track the tank balance from the current candidate start.
//
// When tank becomes negative at station i, the current candidate start cannot
// reach i + 1. Every station between start and i is also impossible:
// - from start to any middle station, tank was nonnegative, otherwise we would
//   have reset earlier
// - starting from that middle station loses the gas accumulated before it
// - so it cannot do better than start did when trying to pass i
//
// Therefore the next possible candidate is i + 1.
//
// Time: O(n)
// Space: O(1)

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let sum_gas: i32 = gas.iter().sum();
    let sum_cost: i32 = cost.iter().sum();

    if sum_cost > sum_gas {
        return -1;
    }

    let mut tank = 0;
    let mut start = 0;
    for i in 0..gas.len() {
        let diff = gas[i] - cost[i];
        tank += diff;

        if tank < 0 {
            tank = 0;
            start = i + 1;
        }
    }

    start as i32
}
