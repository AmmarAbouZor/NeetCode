# Two Pointers Notes

## General pattern

Use two pointers when input is sorted or when checking from both ends.

Common moves:

- sum too small: move left forward
- sum too large: move right backward
- invalid left char: move left
- invalid right char: move right

Time is usually `O(n)` because each pointer moves in one direction.

## Valid Palindrome

Use byte pointers under ASCII constraints.

Skip non-alphanumeric chars:

```rust
if !left_char.is_ascii_alphanumeric() { left += 1; }
if !right_char.is_ascii_alphanumeric() { right -= 1; }
```

Compare with:

```rust
eq_ignore_ascii_case
```

Handle empty string before `len() - 1`, or use `saturating_sub`.

Time: `O(n)`, space: `O(1)`.

## Two Sum II: sorted input

Return 1-indexed indices, not values.

```rust
if sum == target {
    return vec![(left + 1) as i32, (right + 1) as i32];
}
```

If sum is smaller, increase `left`; if larger, decrease `right`.

Time: `O(n)`, space: `O(1)`.

## Three Sum

Sort first, then fix one value and run two-sum on the suffix.

Skip duplicate fixed values:

```rust
if i > 0 && nums[i] == nums[i - 1] {
    continue;
}
```

After finding a triplet, move both pointers and skip duplicate left/right values.

Time: `O(n^2)`. Extra space excluding output: `O(1)`.

Avoid using `HashSet` for duplicate triplets when sorted duplicate-skipping is enough.

## Container With Most Water

Start with pointers at both ends. Area is limited by the shorter height:

```rust
area = area.max(min(height[left], height[right]) * width);
```

Move the pointer with the smaller height. Moving the taller one cannot improve the limiting height while width shrinks.

Time: `O(n)`, space: `O(1)`.

## Interview reminders

- Confirm whether output is values or indices.
- Confirm zero-based vs one-based indices.
- Sorting changes complexity to at least `O(n log n)` and may mutate input.
- Watch unsigned underflow with `right = len - 1` on empty input.
