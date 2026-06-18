# Sliding Window Notes

## General pattern

Use two pointers when maintaining a contiguous range.

```rust
for right in 0..n {
    add nums[right];

    while window_invalid {
        remove nums[left];
        left += 1;
    }

    update_answer();
}
```

Each pointer moves forward at most `n` times, so nested `while` can still be `O(n)`.

## Best Time to Buy and Sell Stock

Keep `left` at best buy day and move `right` as sell day.

If profit is negative, current sell day is a better buy day:

```rust
left = right;
```

Time: `O(n)`, space: `O(1)`.

## Longest Substring Without Repeating Characters

Set version:

- expand `right`
- while duplicate exists, remove from left

Time: `O(n)`, space: `O(m)` where `m` is max unique chars in window.

HashMap version can jump `left` directly:

```rust
if let Some(prev) = last_seen.insert(byte, right) {
    if prev >= left {
        left = prev + 1;
    }
}
```

## Longest Repeating Character Replacement

Window is valid if:

```rust
window_len - max_char_count <= k
```

With `[0; 26]`, recomputing max is `O(26)`, so total time is `O(n)`.

Optimized version tracks `max_count`; it may become stale, but still works because it only delays shrinking and does not miss the best length.

## Permutation in String

Fixed-size window of `s1.len()` over `s2`.

Use two count arrays:

```rust
target == window
```

Array comparison is `O(26)`, treated as constant.

Time: `O(n)`, space: `O(1)`.

## Interview notes

- Ask whether input alphabet is ASCII/lowercase/uppercase.
- If using bytes, mention it assumes ASCII constraints.
- Report worst-case space, not average window size.
