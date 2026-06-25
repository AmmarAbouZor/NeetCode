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

## Hand of Straights

Always start from the smallest remaining card.

That card cannot be placed later in another group because there is no smaller remaining card to start that group. So it must start:

```text
first, first + 1, ..., first + group_size - 1
```

Use a `BTreeMap` for sorted counts:

```rust
while !counts.is_empty() {
    let first = *counts.keys().next().unwrap();
    for card in first..first + group_size {
        decrement count[card]
    }
}
```

If any needed card is missing, return `false`.

Time: `O(n log u)`, where `u` is number of unique cards. Space: `O(u)`.

## Merge Triplets to Form Target

Merge uses max per position, so any triplet with a value greater than target is unusable.

```rust
if triplet[i] > target[i] {
    skip triplet;
}
```

For valid triplets, track whether each target position can be matched:

```rust
matches[i] |= triplet[i] == target[i];
```

If all three positions are matched by valid triplets, merging them can form the target.

Time: `O(n)`. Space: `O(1)`.

## Partition Labels

First record the last position of each character.

Then scan and keep extending the current partition end:

```rust
end = end.max(last_pos[ch]);
```

When current index reaches `end`, every character seen in this partition ends inside it, so the partition is complete.

```rust
if idx == end {
    res.push(end - start + 1);
    start = idx + 1;
}
```

Time: `O(n)`. Space: `O(26) = O(1)`.

## Valid Parenthesis String

Do not choose what `'*'` means immediately. Track a range of possible unmatched opens.

```text
left_min = smallest possible unmatched '('
left_max = largest possible unmatched '('
```

Updates:

- `'('`: both increase
- `')'`: both decrease
- `'*'`: `left_min -= 1`, `left_max += 1`

For `'*'`:

- as `')'`, it can reduce unmatched opens
- as `'('`, it can increase unmatched opens
- as empty, it stays inside the range

If `left_max < 0`, every interpretation has too many `')'`.

Clamp `left_min` to zero because unmatched opens cannot be negative:

```rust
left_min = left_min.max(0);
```

At the end, valid if zero unmatched opens is possible:

```rust
left_min == 0
```

Time: `O(n)`. Space: `O(1)`.
