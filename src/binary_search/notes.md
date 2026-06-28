# Binary Search Notes

## Core template: exclusive right bound

Prefer `[left, right)` in Rust to avoid `usize` underflow.

```rust
let mut left = 0;
let mut right = nums.len();

while left < right {
    let mid = left + (right - left) / 2;

    match nums[mid].cmp(&target) {
        Ordering::Equal => return Some(mid),
        Ordering::Less => left = mid + 1,
        Ordering::Greater => right = mid,
    }
}
```

Use:

```rust
mid = left + (right - left) / 2
```

instead of `(left + right) / 2` to avoid overflow.

## Binary search over answer

Use when the question asks for the minimum/maximum value satisfying a condition.

Pattern:

```rust
while left < right {
    let mid = left + (right - left) / 2;

    if can_do(mid) {
        right = mid;
    } else {
        left = mid + 1;
    }
}
```

Return `left`.

## Koko Eating Bananas

Search speed `k` from `1..=max(piles)`.

Hours for a pile:

```rust
(pile + k - 1) / k
```

This is integer ceiling division. Time: `O(n log m)`, where `m = max(piles)`. Space: `O(1)`.

**Note:**
The problem description feels like DP or greedy.
However, it's a brute force as try every possible k which is made a binary search as our search range is 1..max_pile_count.

## Search 2D Matrix

Treat matrix as flattened sorted array.

```rust
row = mid / cols;
col = mid % cols;
```

Time: `O(log(rows * cols))`, space: `O(1)`.

## Rotated Sorted Array

At each step, one half is sorted.

```rust
if nums[left] <= nums[mid] {
    // left half sorted
} else {
    // right half sorted
}
```

Then check whether target lies inside that sorted half. Assumes distinct values unless stated otherwise.

## Minimum in Rotated Sorted Array

Compare `mid` with `right`:

```rust
if nums[mid] > nums[right] {
    left = mid + 1;
} else {
    right = mid;
}
```

The minimum is where `left == right`.

## Median of Two Sorted Arrays

Binary search the partition point in the shorter array.

Think of the merged array split into left and right halves. If we choose:

```text
short_cut = number of elements from short array on the left
long_cut = left_len - short_cut
```

A valid partition has:

```text
short_left <= long_right
long_left <= short_right
```

Then every value on the left is `<=` every value on the right.

For odd total length, the left half gets the extra element, so the median is `max(left side)`. For even total length, it is the average of `max(left side)` and `min(right side)`.

Move binary search left if `short_left > long_right`; otherwise move right.

Use sentinels for cuts at array edges: `i32::MIN` for missing left, `i32::MAX` for missing right.

Time: `O(log(min(n, m)))`, space: `O(1)`.

## TimeMap

Store values per key in timestamp order:

```rust
HashMap<String, Vec<(timestamp, value)>>
```

`set`: amortized `O(1)`.

`get`: binary search for the last timestamp `<= query`.

```rust
if values[mid].0 <= timestamp {
    ans = values[mid].1.clone();
    left = mid + 1;
} else {
    right = mid;
}
```

Better Rust helper:

```rust
let idx = values.partition_point(|(ts, _)| *ts <= timestamp);
```

Space: `O(N)` total stored entries.
