# Heap Notes

## Binary heap basics

Rust `BinaryHeap` is a max-heap by default.

- `peek`: `O(1)`
- `push`: `O(log n)`
- `pop`: `O(log n)`
- `BinaryHeap::from(vec)`: heapify once in `O(n)`

Use `Reverse<T>` for a min-heap:

```rust
BinaryHeap<Reverse<i32>>
```

Duplicates are allowed. A heap is not stable and does not keep all items sorted.

## Kth largest patterns

For kth largest in an array/stream, keep a min-heap of size `k`.

Invariant:

> The heap stores the current k largest values. The root is the smallest among them, so it is the kth largest overall.

- Constructor over `n` values: `O(n log k)`, space `O(k)`.
- `add`: `O(log k)`.

A max-heap cannot directly return kth largest without popping `k` items.

## Quickselect follow-up

Quickselect partitions around a pivot and continues only on the side containing the target index.

- Average time: `O(n)`
- Worst case: `O(n^2)`
- Extra space: `O(1)` if in-place

For kth largest with descending partition:

```rust
let target = k - 1;
```

For k closest points, first k closest occupy indices `0..k`, so target index is `k - 1`.

## K Closest Points

Heap approach:

- Keep a max-heap of size `k` by squared distance.
- Push point, pop if heap grows past `k`.
- Time: `O(n log k)`, space: `O(k)`.

Use squared distance, not `sqrt`:

```rust
x * x + y * y
```

Distance order is preserved and avoids floating point.

## Last Stone Weight

Use max-heap. Pop two largest, push their difference if non-zero.

- Heapify: `O(n)`
- Smashing loop: `O(n log n)`
- Space: `O(n)`

## Task Scheduler

General simulation uses:

- max-heap for available task counts
- queue for cooling tasks with ready time

Each cycle:

1. run highest-count available task if possible
2. push it to cooldown if count remains
3. move ready cooldown tasks back to heap
4. if heap is empty, jump to next ready time

Time: `O(N + T log A)`, where `T` is final schedule length and `A` is unique task types. For uppercase tasks, `A <= 26`, so this is effectively `O(N + T)`. Space: `O(A)`.

For this exact LeetCode problem, a greedy formula also exists, but heap + queue is the general scheduler pattern.
