# Greedy Notes

## General idea

Greedy works when a local decision can safely discard other options.

For interviews, explain the invariant:

- what choice are we making now?
- why is it safe?
- what state do we carry forward?

## Maximum Subarray

Kadane's algorithm.

State:

```text
cur_sum = best subarray sum ending at current index
```

At each number:

```rust
cur_sum = num.max(cur_sum + num);
max_sum = max_sum.max(cur_sum);
```

Greedy intuition: if the previous sum hurts the current number, start fresh.

Time: `O(n)`. Space: `O(1)`.

## Jump Game

Work backwards from the goal.

```text
goal = leftmost index known to reach the end
```

If index `i` can reach `goal`, then `i` becomes the new goal.

```rust
if i + nums[i] >= goal {
    goal = i;
}
```

Return `goal == 0`.

Time: `O(n)`. Space: `O(1)`.

## Jump Game II

Think of each jump as a window of reachable indices.

- current window `[l, r]` = all indices reachable with current jump count
- scan the window to find `furthest`
- next window is `[r + 1, furthest]`
- increment jumps after finishing each window

```rust
while r < nums.len() - 1 {
    let mut furthest = 0;
    for i in l..=r {
        furthest = furthest.max(i + nums[i] as usize);
    }
    l = r + 1;
    r = furthest;
    jumps += 1;
}
```

Time: `O(n)`. Space: `O(1)`.

## Gas Station

First check total feasibility:

```rust
if sum_gas < sum_cost {
    return -1;
}
```

Then scan with a candidate `start` and current `tank`.

If `tank < 0` at station `i`, then `start` cannot reach `i + 1`. Any station between `start` and `i` also cannot work, so reset:

```rust
tank = 0;
start = i + 1;
```

Why skip the middle stations: starting in the middle loses the nonnegative gas accumulated before it, so it cannot do better than the failed start.

Time: `O(n)`. Space: `O(1)`.
