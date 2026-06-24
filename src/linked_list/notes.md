# Linked List Notes

## Rust ownership pattern

For `Option<Box<ListNode>>`, use `.take()` to move links safely.

```rust
while let Some(mut node) = current {
    current = node.next.take();
    node.next = prev;
    prev = Some(node);
}
```

`.take()` replaces the option with `None` and gives ownership of the old value.

## Reverse Linked List

Iterative reverse:

1. take current node
2. detach next
3. point current node to previous
4. advance previous/current

Time: `O(n)`, space: `O(1)`.

## Merge Two Sorted Lists

Use a dummy head and mutable `tail`.

Key pattern:

```rust
tail.next = list1.take();
tail = tail.next.as_mut().unwrap();
list1 = tail.next.take();
```

This attaches one node, moves tail to it, then detaches the rest back into `list1`.

Use `list1.or(list2)` to attach the remaining list. Time: `O(n + m)`, space: `O(1)`.

## Reorder List

Safe Rust approach:

1. count length
2. split at middle
3. reverse second half
4. merge alternating nodes

Fast/slow mutable pointer style is natural in C++, but awkward in safe Rust because it wants mutable and immutable access into the same list.

Extra pass is fine: still `O(n)` time and `O(1)` space.

## Remove Nth Node From End

Two-pass version is simple in Rust:

1. count length
2. convert `n` from end into index from front: `target = len - n`
3. use a dummy node and stop at the node before `target`
4. skip target with `next.take()`

Dummy node handles removing the head cleanly.

Time: `O(n)`, space: `O(1)`.

## Add Two Numbers

Do the addition digit by digit, like hand addition.

State:

```text
add(l1, l2, carry)
```

Each step:

```rust
sum = val1 + val2 + carry
node.val = sum % 10
carry = sum / 10
```

Base case: both lists are empty and `carry == 0`.

Do not convert the whole list into an integer; long inputs can overflow.

Time: `O(max(n, m))`. Space: `O(max(n, m))` for output and recursion stack.

## Copy List With Random Pointer

Values are not guaranteed unique, so key by node identity, not value.

Safe Rust approach with `Rc<RefCell<Node>>`:

1. first pass: clone each node value and store original pointer -> copied node
2. second pass: assign copied `next` and `random` using the map

```rust
map.insert(Rc::as_ptr(&node), copied);
```

Using `Rc::as_ptr` as a HashMap key is fine when it is only used as identity and not dereferenced.

Time: `O(n)`, space: `O(n)`.

## LRU Cache

Expected interview design for `O(1)` get/put:

```text
HashMap<key, node>
doubly linked list ordered by recency
```

In Rust, a safe approach is an arena-style linked list:

```rust
Vec<Entry>
HashMap<i32, usize>
Entry { key, value, prev: Option<usize>, next: Option<usize> }
```

Keep:

- `head` = least recently used
- `tail` = most recently used

On `get` or updating existing key, move the node to `tail`.

On insert when full, evict `head`.

Core helpers:

- `detach(idx)`: link previous and next around the node
- `push_back(idx)`: attach node as most recent
- `evict_lru()`: detach `head` and remove its key from the map

Time: `O(1)` average per operation. Space: `O(capacity)`.

## Counting list length

Nice immutable traversal:

```rust
std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count()
```

## Cycle Detection with raw pointers

Floyd algorithm:

- slow moves 1 step
- fast moves 2 steps
- if they meet, cycle exists

Raw pointer version should be `unsafe fn` because non-null does not mean valid. Caller must guarantee live, aligned, initialized nodes and valid `next` pointers.

Keep unsafe blocks small and document invariants.

## Find Duplicate Number

Array values act like pointers:

```rust
next = nums[index]
```

Values are `[1, n]`, so index `0` starts outside the cycle and eventually enters it. Duplicate value is the cycle entry.

Floyd stages:

1. find slow/fast intersection
2. move one pointer from start and one from intersection; meeting point is duplicate

Time: `O(n)`, space: `O(1)`, no mutation.

XOR only works when the duplicate appears exactly twice; it does not handle the general constraints.
