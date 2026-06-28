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

**Note:** Stating that chars are only English uppercase is an indicator to think about simpler algorithm before jumping into DP or greedy.

Optimized version tracks `max_count`; it may become stale, but still works because it only delays shrinking and does not miss the best length.

## Permutation in String

Fixed-size window of `s1.len()` over `s2`.

Use two count arrays:

```rust
target == window
```

Array comparison is `O(26)`, treated as constant.

Time: `O(n)`, space: `O(1)`.

## Minimum Window Substring

Sliding window with frequency counts.

Build `target` counts from `t`, then expand `right` over `s` and update `window` counts.

Track validity with distinct-character counts:

```text
need = number of distinct required chars
have = number of required chars whose window count meets target count
```

When `have == need`, the window contains all required chars with enough frequency. Shrink from the left while valid, updating the best answer before removing `s[left]`.

Important detail: increment `have` only when a char count reaches exactly the target count, and decrement it only when removing from the left makes the count fall below target.

Time: `O(s.len() + t.len())` average with `HashMap`. Space: `O(distinct chars in s and t)`.

If using byte indexing, mention the ASCII input constraint.

## Sliding Window Maximum

Use a monotonic decreasing deque of indices.

The front of the deque is always the max candidate for the current window.

Before pushing `right`, pop from the back while values are smaller than `nums[right]`:

```rust
while nums[back] < nums[right] {
    pop_back();
}
```

Those smaller values can never become the max while `nums[right]` is in the window.

Once a full window exists, remove indices from the front if they are outside the window:

```rust
left = right + 1 - k
```

Then `deque.front()` is the max index for that window.

Using `<` keeps duplicate values in the deque, which is fine. The older duplicate expires first.

Time: `O(n)`, because each index is pushed and popped at most once. Space: `O(k)`.

## Interview notes

- Ask whether input alphabet is ASCII/lowercase/uppercase.
- If using bytes, mention it assumes ASCII constraints.
- Report worst-case space, not average window size.
