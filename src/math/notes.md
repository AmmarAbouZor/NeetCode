# Math Notes

## General patterns

Math problems are often about using the right representation:

- digits from right to left with `% 10` and `/ 10`
- matrix boundaries for traversal
- transpose/reverse for rotation
- counting coordinates in a map
- binary exponentiation for powers

Keep the implementation boring. Most bugs come from edge cases: overflow, leading zeros, empty input, or `usize` underflow.

## Plus One

Same as hand addition.

Start from the right:

- if digit is `< 9`, increment and return
- if digit is `9`, set it to `0` and carry left
- if all digits were `9`, insert `1` at the front

```rust
for idx in (0..digits.len()).rev() {
    if digits[idx] < 9 {
        digits[idx] += 1;
        return digits;
    }
    digits[idx] = 0;
}

digits.insert(0, 1);
```

Time: `O(n)`. Space: `O(1)` extra, excluding output resize.

## Pow(x, n)

Fast exponentiation uses the binary representation of `n`.

Example:

```text
x^13 = x^8 * x^4 * x^1
13 = 1101b
```

Keep:

- `result`: product of powers included so far
- `base`: current power of `x`, moving `x, x^2, x^4, x^8, ...`
- `power`: remaining exponent bits

Loop:

```rust
if power % 2 == 1 {
    result *= base;
}
base *= base;
power /= 2;
```

For negative exponents, use `1 / x` as the base and make the power positive.

Convert `n` to `i64` before negating so `i32::MIN` is safe.

Time: `O(log n)`. Space: `O(1)`.

## Multiply Strings

Manual multiplication with a result vector.

For digits `num1[i]` and `num2[j]`, their product contributes to:

```rust
res[i + j + 1]
```

Result length is at most:

```text
num1.len() + num2.len()
```

Steps:

1. multiply every digit pair into `res[i + j + 1]`
2. propagate carry from right to left
3. skip leading zeros
4. convert digits back to chars

Return early if either input is `"0"`.

Time: `O(n1 * n2)`. Space: `O(n1 + n2)`.

## Happy Number

Repeatedly replace `n` with the sum of squares of its digits.

If the sequence reaches `1`, it is happy. Otherwise it eventually cycles.

HashSet version:

- store seen values
- if a value repeats, there is a cycle

Floyd version:

- slow moves one `calc_squares` step
- fast moves two steps
- if fast reaches `1`, return true
- if slow meets fast, cycle without `1`

Digit helper:

```rust
while n != 0 {
    res += (n % 10).pow(2);
    n /= 10;
}
```

Time: `O(log n)` initially, effectively bounded after values shrink. Space: `O(1)` with Floyd, `O(k)` with HashSet.

## Rotate Image

For 90 degrees clockwise rotation of an `n x n` matrix.

Two equivalent approaches:

1. reverse rows vertically, then transpose
2. transpose, then reverse each row

Most common:

```rust
// transpose
for i in 0..n {
    for j in i + 1..n {
        swap matrix[i][j] and matrix[j][i]
    }
}

// reverse each row
for row in matrix {
    row.reverse();
}
```

Using a temp variable is simple in Rust because swapping two indexed matrix cells can cause borrow issues.

Time: `O(n^2)`. Space: `O(1)`.

## Spiral Matrix

Maintain four boundaries:

```text
top, bottom, left, right
```

Each layer processes four sides in order:

1. top row, left to right
2. right column, top to bottom
3. bottom row, right to left
4. left column, bottom to top

After each side, move that boundary inward.

Recheck conditions before bottom row and left column because single-row/single-column layers can disappear.

Time: `O(m * n)`. Space: `O(1)` excluding result.

## Set Matrix Zeroes

Simple version uses sets of zero rows and columns: `O(m + n)` space.

O(1) space trick:

- use first row as column markers
- use first column as row markers
- use a separate boolean for whether the first row should be zeroed
- let `matrix[0][0]` represent whether the first column should be zeroed

When finding a zero at `(r, c)`:

```rust
matrix[0][c] = 0;
matrix[r][0] = 0;
```

Then zero interior cells based on markers:

```rust
if matrix[0][c] == 0 || matrix[r][0] == 0 {
    matrix[r][c] = 0;
}
```

Finally zero first column and first row if needed.

Time: `O(m * n)`. Space: `O(1)`.

## Detect Squares

Store point counts because duplicate points are allowed:

```rust
HashMap<(x, y), count>
```

For a query point `(x, y)`, treat each stored point `(px, py)` as the opposite diagonal corner.

It can form an axis-aligned square only if:

```text
abs(px - x) == abs(py - y)
abs(px - x) != 0
```

The other two corners must be:

```text
(x, py)
(px, y)
```

Add combinations:

```rust
count += diag_count * corner1_count * corner2_count;
```

`add`: `O(1)` average. `count`: `O(p)`, where `p` is unique points. Space: `O(p)`.
