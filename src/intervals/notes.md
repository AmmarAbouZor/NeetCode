# Intervals Notes

## General pattern

Most interval problems start with sorting by start time:

```rust
intervals.sort_unstable_by_key(|i| i[0]);
```

Then scan once and compare the current interval with the last relevant interval.

Overlap for closed intervals:

```text
cur_start <= prev_end
```

Non-overlap:

```text
cur_start >= prev_end
```

For meeting-style intervals, `[1, 3]` and `[3, 5]` do not overlap.

## Insert Interval

Scan sorted intervals and handle three cases:

1. current interval is before new interval: push current
2. current interval is after new interval: insert merged new interval once, then push current
3. overlap: merge into new interval

```rust
start = start.min(cur_start);
end = end.max(cur_end);
```

Time: `O(n)`. Space: `O(n)` for output.

## Merge Intervals

Sort by start. Keep `results` with merged intervals.

For each interval, compare it with the last merged interval:

```rust
if start <= last[1] {
    last[1] = last[1].max(end);
} else {
    results.push(interval);
}
```

Time: `O(n log n)`. Space: `O(n)` for output.

## Non-overlapping Intervals

Sort by start. Track the end of the interval being kept.

On overlap, remove one interval and keep the smaller end:

```rust
count += 1;
last_end = last_end.min(end);
```

Keeping the smaller end leaves more room for future intervals.

Time: `O(n log n)`. Space: `O(1)` excluding sort internals.

## Meeting Rooms

Sort by start and compare adjacent intervals.

```rust
for win in intervals.windows(2) {
    if win[1].start < win[0].end {
        return false;
    }
}
```

Time: `O(n log n)`. Space: `O(1)` excluding sort internals.

## Meeting Rooms II

Use a min-heap of room end times. The earliest-free room is on top.

```rust
if heap.peek().is_some_and(|end| *end <= interval.start) {
    heap.pop();
}
heap.push(interval.end);
```

In Rust, use `Reverse<i32>` because `BinaryHeap` is a max-heap by default.

If the earliest room is not free, no room is free, so allocate another room.

Time: `O(n log n)`. Space: `O(m)`, where `m` is the number of rooms.

## Meeting Rooms III

Two-heap simulation.

Keep:

```text
available = free room ids, ordered by smallest room id
busy = occupied rooms as (free_time, room_id), ordered by earliest free time
```

Process meetings sorted by start time.

For each meeting:

1. release every room with `free_time <= start`
2. if a room is available, use the smallest room id
3. otherwise delay the meeting until the earliest busy room becomes free

When delaying, keep the original duration:

```text
new_end = earliest_free_time + (end - start)
```

The busy heap stores `(free_time, room_id)`, so ties use the smaller room id automatically.

For the final answer, scan counts and update only on a strictly larger count. This keeps the smallest room id on ties.

Time: `O(m log m + m log n)`, where `m = meetings.len()` and `n = rooms`. Space: `O(n)`.
