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

## Subarray Sum Equals K

Prefix sum + hashmap of previous prefix counts.

For current prefix sum `prefix`, a previous prefix `needed` forms a subarray ending here with sum `k` when:

```text
prefix - needed = k
needed = prefix - k
```

Keep:

```text
pref_count[p] = how many times prefix sum p appeared before
```

Initialize `pref_count[0] = 1` so subarrays starting at index `0` are counted.

Works with negative numbers, unlike sliding window.

Time: `O(n)`. Space: `O(n)`.

## Group Anagrams

Use character-count array as the group key:

```rust
let mut counts = [0; 26];
counts[(b - b'a') as usize] += 1;

// Map
HashMap<[usize; 26], Vec<String>>
```

This avoids comparing each string against previous groups. Time: `O(n * L)`.

## Top K Frequent

Options:

- Bucket sort by frequency: `O(n)` time, `O(n)` space.
- Min-heap of size `k`: `O(n log k)` time, `O(k)` heap space plus counts.
- Max-heap of all unique values: `O(n + m log m)` where `m` is unique count.

Bucket sort is best when frequencies are bounded by `nums.len()`.

## Product Except Self

Use separate prefix and suffix arrays, then multiply the values around each index.

```rust
prefix[i] = nums[0] * ... * nums[i];
suffix[i] = nums[i] * ... * nums[n - 1];

left = if i == 0 { 1 } else { prefix[i - 1] };
right = suffix.get(i + 1).copied().unwrap_or(1);
res[i] = left * right;
```

No division needed. Time: `O(n)`, space: `O(n)` for prefix, suffix, and output.

Note: It's still possible to calculate prefix and suffix directly in output array making
Space `O(1)`.

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

We need to catch duplicates in each row, column and 3x3 square.
It's possible to do it in one loop

```rust
let square_idx = (r / 3) * 3 + c / 3;
```

For a fixed 9x9 board, time and space are `O(1)` technically. If generalized to `n x n`, scan is `O(n^2)`.

Bitmask variant: store seen digits as bits instead of sets.
