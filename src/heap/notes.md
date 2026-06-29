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

## Median from Data Stream

Use two heaps:

- `lower`: max-heap for the smaller half
- `upper`: min-heap for the larger half, using `Reverse`

Invariants:

```text
every value in lower <= every value in upper
lower.len() == upper.len() or lower.len() == upper.len() + 1
```

Keep `lower` as the heap with the extra element when the total count is odd.

Median:

- odd count: `lower.peek()`
- even count: average of `lower.peek()` and `upper.peek()`

`add_num`: `O(log n)`. `find_median`: `O(1)`. Space: `O(n)`.

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

## Design Twitter

Store per user:

- set of followees
- latest 10 tweets as `(timestamp, tweet_id)`

Keeping only 10 tweets per user is safe because the feed only asks for 10 tweets. If a user has more than 10 tweets, older tweets cannot beat that same user's 10 newer tweets.

For `get_news_feed`, scan:

- current user's latest tweets
- each followee's latest tweets

Keep the 10 newest tweets in a min-heap:

```rust
heap.push(Reverse((timestamp, tweet_id)));
if heap.len() > 10 {
    heap.pop();
}
```

Pop gives oldest-to-newest among the kept tweets, so reverse the result at the end.

Time:

- `post_tweet`: `O(1)`
- `follow` / `unfollow`: `O(1)` average
- `get_news_feed`: `O(f * 10 * log 10) = O(f)`, where `f` is number of followees

Space: `O(u + e)`, where `u` is users and `e` is follow relationships. Feed call uses `O(10)` extra space.
