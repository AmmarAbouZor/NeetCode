# Arrays and Hashing Notes

## General patterns

- HashSet answers "have I seen this value before?" in average `O(1)`.
- HashMap answers "what count/index/group belongs to this key?" in average `O(1)`.
- If constraints say lowercase English letters, prefer `[usize; 26]` over `HashMap`.
- Always read constraints. They often reveal fixed alphabets, board sizes, or valid input assumptions.

## Contains Duplicate

```rust
if !set.insert(x) {
    return true;
}
```

`HashSet::insert` returns `false` when the value already exists.

## Valid Anagram

- General Unicode/string case: use `HashMap<char, count>`.
- Lowercase English only: use `[i32; 26]` and index with `b - b'a'`.
- One-map approach: increment for `s`, decrement while scanning `t`.
- Time: `O(n)`, space: `O(k)` where `k` is alphabet size.

## Two Sum unsorted

Store complements or previous numbers in a `HashMap`.

```rust
if let Some(prev_idx) = map.get(&num) {
    return answer;
}
map.insert(target - num, idx);
```

Time: `O(n)`, space: `O(n)`.

## Group Anagrams

Use character-count array as the group key:

```rust
let mut counts = [0; 26];
counts[(b - b'a') as usize] += 1;
```

This avoids comparing each string against previous groups. Time: `O(n * L)`.

## Top K Frequent

Options:

- Bucket sort by frequency: `O(n)` time, `O(n)` space.
- Min-heap of size `k`: `O(n log k)` time, `O(k)` heap space plus counts.
- Max-heap of all unique values: `O(n + m log m)` where `m` is unique count.

Bucket sort is best when frequencies are bounded by `nums.len()`.

## Product Except Self

Use output vector for prefix products, then multiply suffix into it.

```rust
res[i] = prefix;
prefix *= nums[i];

res[i] *= suffix;
suffix *= nums[i];
```

No division needed. Time: `O(n)`, extra space excluding output: `O(1)`.

## Longest Consecutive Sequence

Put all numbers in a set. Start counting only from sequence starts:

```rust
if set.contains(&(num - 1)) {
    continue;
}
```

Each sequence is expanded once overall. Time: `O(n)`, space: `O(n)`.

## Encode and Decode Strings

Use length prefix to handle any characters in the string:

```text
len#string
```

Important iterator note: use `by_ref()` when consuming part of an iterator and continuing later.

## Valid Sudoku

Track each row, column, and 3x3 square.

```rust
let square_idx = (r / 3) * 3 + c / 3;
```

For a fixed 9x9 board, time and space are `O(1)` technically. If generalized to `n x n`, scan is `O(n^2)`.

Bitmask variant: store seen digits as bits instead of sets.
