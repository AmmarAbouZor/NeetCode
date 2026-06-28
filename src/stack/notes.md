# Stack Notes

## General stack uses

Use a stack when the latest unresolved item is the first one that can be resolved.

Common patterns:

- matching brackets
- expression evaluation
- monotonic stack
- previous/next greater/smaller element

## Valid Parentheses

Push opening brackets. On closing bracket, pop and check pair.

At the end, stack must be empty.

Time: `O(n)`, space: `O(n)` worst case.

## Min Stack

Use two stacks:

- values stack
- min-at-this-depth stack

On push, push `min(current_value, previous_min)` to min stack.

All operations are `O(1)`. Space: `O(n)`.

## Reverse Polish Notation

For each token:

- number: push
- operator: pop right, pop left, apply, push result

Order matters for subtraction/division:

```rust
left - right
left / right
```

Time: `O(n)`, space: `O(n)`.

## Daily Temperatures

Monotonic decreasing stack of unresolved indices.

When current temperature is warmer than stack top:

```rust
res[prev_idx] = current_idx - prev_idx;
```

Each index is pushed and popped at most once. Time: `O(n)`, space: `O(n)`.

## Largest Rectangle in Histogram

Monotonic increasing stack of bar indices.

When the current bar is shorter than the stack top, the top bar cannot extend past the current index. Pop it and compute the largest rectangle where that popped bar is the limiting height.

After popping, the new stack top is the nearest shorter bar to the left. The current index is the first shorter bar to the right.

```rust
width = if let Some(left) = stack.last() {
    idx - left - 1
} else {
    idx
};
```

Add a sentinel `0` height at the end to flush the stack.

Each index is pushed and popped at most once. Time: `O(n)`, space: `O(n)`.

## Car Fleet

**Note**: Solution doesn't use a stack as we only need to track the latest fleet.

Sort cars by position. Scan from closest to target backwards.

Compute time to target:

```rust
time = (target - position) / speed
```

If a car behind has time `<=` current fleet time, it joins that fleet. If time is greater, it starts a new fleet.

Stack of fleet times works, but only the latest slowest fleet time is needed.

Time: `O(n log n)` due to sorting. Space: `O(n)` for cars, `O(1)` fleet tracking if optimized.
