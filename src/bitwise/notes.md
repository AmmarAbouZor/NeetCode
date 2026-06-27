# Bitwise Notes

## Core identities

XOR cancels equal values:

```text
x ^ x = 0
x ^ 0 = x
```

Useful for problems where every value appears twice except one.

Check a bit with shift and AND:

```rust
if (n & (1 << i)) != 0 {
    // bit i is set
}
```

Set a bit in the result:

```rust
res |= bit << pos;
```

## Single Number

XOR all numbers. Pairs cancel and the single value remains.

```rust
let mut res = 0;
for num in nums {
    res ^= num;
}
```

Time: `O(n)`. Space: `O(1)`.

## Number of 1 Bits

Simple fixed-width scan for `u32`:

```rust
for i in 0..32 {
    if (n & (1 << i)) != 0 {
        count += 1;
    }
}
```

Time: `O(32) = O(1)`. Space: `O(1)`.

Alternative trick:

```rust
n &= n - 1;
```

This removes the lowest set bit each iteration.

## Counting Bits

Simple version: count bits for every number from `0..=n`.

Time: `O(32 * n) = O(n)`. Space: `O(n)` for output.

DP recurrence worth remembering:

```text
bits[i] = bits[i >> 1] + (i & 1)
```

`i >> 1` removes the last bit, and `i & 1` tells whether the removed bit was `1`.

## Reverse Bits

Read each bit from the input and place it at the mirrored position.

```rust
for i in 0..32 {
    let bit = (n >> i) & 1;
    res |= bit << (31 - i);
}
```

Time: `O(32) = O(1)`. Space: `O(1)`.

## Missing Number

Numbers are from `0..=n` with one missing.

XOR all indices and values. Existing numbers cancel, missing number remains.

```rust
let mut res = nums.len();
for (idx, num) in nums.into_iter().enumerate() {
    res ^= idx;
    res ^= num as usize;
}
```

Time: `O(n)`. Space: `O(1)`.

## Sum of Two Integers

Binary addition without `+` uses two operations:

```text
sum without carry = a ^ b
carry = (a & b) << 1
```

Repeat until carry is `0`:

```rust
while b != 0 {
    let sum_no_carry = a ^ b;
    let carry = (a & b) << 1;
    a = sum_no_carry;
    b = carry;
}
```

Time: `O(32) = O(1)`. Space: `O(1)`.

## Reverse Integer

Take digits from the end of `x` and append them to the result.

```rust
let digit = x % 10;
x /= 10;
res = res * 10 + digit;
```

Use checked arithmetic because reversing can overflow `i32`:

```rust
res.checked_mul(10).and_then(|r| r.checked_add(digit))
```

Rust keeps the sign for `%`, so negative numbers work with the same logic:

```text
-123 % 10 = -3
-123 / 10 = -12
```

Return `0` on overflow.

Time: `O(1)` because `i32` has a fixed number of digits. Space: `O(1)`.
