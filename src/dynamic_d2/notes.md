# 2D Dynamic Programming Notes

## General 2D DP checklist

Most 2D DP problems use two dimensions for:

- two indices, usually two strings
- index + amount/sum/state
- row + column in a grid

Start with the full table first.

1. Define what `dp[i][j]` means.
2. Decide what smaller states can lead into `dp[i][j]`.
3. Fill base cases for empty prefix / first row / first column.
4. Optimize space only after the recurrence is clear.

A good trick: ask what the last step or last character was.

## Unique Paths

Grid DP.

State:

```text
dp[r][c] = number of ways to reach cell (r, c)
```

Each cell can be reached from above or left:

```text
dp[r][c] = dp[r - 1][c] + dp[r][c - 1]
```

First row and first column are all `1` because there is only one straight path.

Row-optimized version:

```rust
let mut dp = vec![1; n];

for _ in 1..m {
    for c in 1..n {
        dp[c] += dp[c - 1];
    }
}
```

`dp[c]` is the value from above before overwrite. `dp[c - 1]` is the value from left in the current row.

Time: `O(m * n)`. Space: `O(n)` with row DP, `O(m * n)` with full grid.

## Unique Paths II

Same grid DP, but obstacles make cells unreachable.

Full-table state:

```text
dp[r][c] = number of ways to reach cell (r, c)
```

If `obstacle_grid[r][c] == 1`, keep `dp[r][c] = 0`. Otherwise add ways from top and left.

Unlike Unique Paths I, the first row and column are not automatically `1` because an obstacle can block the path.

Space-optimized state:

```text
dp[c] = number of ways to reach column c in the current row
```

Before update, `dp[c]` is ways from top. After update, `dp[c - 1]` is ways from left in the current row.

For an obstacle, set `dp[c] = 0` so cells below cannot use it as a path from above.

Time: `O(m * n)`. Space: `O(n)` optimized, `O(m * n)` full table.

## Minimum Path Sum

Grid DP.

State:

```text
dp[r][c] = minimum path sum to reach cell (r, c)
```

Each cell can only be reached from above or left:

```text
dp[r][c] = grid[r][c] + min(dp[r - 1][c], dp[r][c - 1])
```

For full-table DP, either initialize the first row/column explicitly, or use sentinel `inf` values and one uniform loop.

Space-optimized state:

```text
dp[c] = minimum path sum to reach column c in the current row
```

Before update, `dp[c]` is the value from top. After update, `dp[c - 1]` is the value from left in the current row.

Time: `O(m * n)`. Space: `O(n)` optimized, `O(m * n)` full table.

## Longest Common Subsequence

DP over prefixes of two strings.

State:

```text
dp[i][j] = LCS length using first i chars of text1 and first j chars of text2
```

If chars match, use both:

```rust
dp[i][j] = 1 + dp[i - 1][j - 1]
```

If chars differ, skip one char from either string:

```rust
dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
```

Use `m + 1` by `n + 1` table so empty prefixes are handled by zeros.

Space optimization: keep previous row and current row.

Time: `O(m * n)`. Space: `O(m * n)` full table, `O(n)` with two rows.

## Distinct Subsequences

DP over prefixes.

Bottom-up state:

```text
dp[i][j] = number of ways to form t[0..j] using s[0..i]
```

Base:

```text
dp[i][0] = 1
```

Empty `t` can be formed from any prefix of `s` by choosing no chars.

Transition for current chars `s[i - 1]` and `t[j - 1]`:

- always skip current `s` char: `dp[i - 1][j]`
- if chars match, also use it: `dp[i - 1][j - 1]`

```text
if s[i - 1] == t[j - 1]:
    dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1]
else:
    dp[i][j] = dp[i - 1][j]
```

Top-down version uses suffix state:

```text
dfs(i, j) = ways to form t[j..] from s[i..]
```

Same choices: skip `s[i]`, and if chars match, include it and advance both indices.

Time: `O(s.len() * t.len())`. Space: `O(s.len() * t.len())`.

## Edit Distance

State:

```text
dp[i][j] = minimum operations to convert word1[0..i] into word2[0..j]
```

Base cases:

```text
dp[i][0] = i  // delete all chars from word1
dp[0][j] = j  // insert all chars from word2
```

If last chars match:

```rust
dp[i][j] = dp[i - 1][j - 1]
```

If last chars differ, think about the final operation:

- insert `word2[j - 1]`: `dp[i][j - 1]`
- delete `word1[i - 1]`: `dp[i - 1][j]`
- replace last char: `dp[i - 1][j - 1]`

```rust
dp[i][j] = 1 + min(insert, delete, replace)
```

The recurrence is easier if you reason backward from the final operation.

Time: `O(m * n)`. Space: `O(m * n)`.

## Regular Expression Matching

DP over prefixes.

State:

```text
dp[i][j] = true if s[0..i] matches p[0..j]
```

Base:

```text
dp[0][0] = true
```

Empty string can match patterns like `a*` or `a*b*` by skipping each `x*` group:

```text
if p[j - 1] == '*':
    dp[0][j] = dp[0][j - 2]
```

For normal chars or `.`:

```text
if p[j - 1] == s[i - 1] || p[j - 1] == '.':
    dp[i][j] = dp[i - 1][j - 1]
```

For `*`, it applies to the previous pattern char `x`:

- zero occurrences: `dp[i][j - 2]`
- one or more occurrences, if `x` matches `s[i - 1]`: `dp[i - 1][j]`

Stay at the same pattern prefix for one-or-more because `x*` may match more chars.

Top-down version uses suffix state:

```text
dfs(i, j) = whether s[i..] matches p[j..]
```

Same choices for `*`: skip the `x*` group, or consume one matching string char and stay on `j`.

Time: `O(s.len() * p.len())`. Space: `O(s.len() * p.len())`.

## Interleaving String

State:

```text
dp[i][j] = true if s3[0..i + j] can be formed using s1[0..i] and s2[0..j]
```

To compute `dp[i][j]`, ask where the last char of `s3[0..i + j]` came from.

From `s1`:

```rust
i > 0 && dp[i - 1][j] && s1[i - 1] == s3[i + j - 1]
```

From `s2`:

```rust
j > 0 && dp[i][j - 1] && s2[j - 1] == s3[i + j - 1]
```

If either is true, `dp[i][j] = true`.

First check lengths:

```rust
if s1.len() + s2.len() != s3.len() {
    return false;
}
```

Time: `O(m * n)`. Space: `O(m * n)`.

## Longest Increasing Path in a Matrix

Top-down DP with DFS + memoization.

State:

```text
dp[r][c] = length of the longest increasing path starting at cell (r, c)
```

From each cell, try 4-direction neighbors with strictly larger values:

```text
matrix[nr][nc] > matrix[r][c]
```

Memoization makes each cell computed once. The answer is the max `dp[r][c]` over all cells.

Strictly increasing moves prevent cycles along a valid path.

Time: `O(rows * cols)`. Space: `O(rows * cols)` for memoization plus recursion stack.

## Burst Balloons

Top-down interval DP with memoization.

Key mental model: choose which balloon is burst last inside an interval, not which balloon is burst first.

Add virtual boundary balloons:

```text
vals = [1] + nums + [1]
```

State:

```text
dfs(left, right) = max coins from bursting all balloons strictly between left and right
```

`left` and `right` are fixed boundary balloons and are not burst in this subproblem.

If `mid` is the last balloon burst inside `(left, right)`, then every other balloon in that interval is already gone. So `mid`'s final neighbors are exactly `left` and `right`:

```text
vals[left] * vals[mid] * vals[right]
```

Transition:

```text
dfs(left, right) = max(
    dfs(left, mid) + vals[left] * vals[mid] * vals[right] + dfs(mid, right)
)
```

Base case:

```text
right == left + 1 => 0
```

There are no balloons strictly between the boundaries.

Memoize by `(left, right)`. There are `O(n^2)` intervals, and each interval tries `O(n)` possible last balloons.

Bottom-up version uses the same state as `dp[left][right]` and fills smaller intervals first by increasing gap.

Time: `O(n^3)`. Space: `O(n^2)`.

## Best Time to Buy/Sell Stock With Cooldown

State:

```text
hold[i] = max profit at end of day i while holding a stock
not_hold[i] = max profit at end of day i while not holding a stock
```

Transitions:

```text
hold[i] = max(hold[i - 1], not_hold[i - 2] - price[i])
not_hold[i] = max(not_hold[i - 1], hold[i - 1] + price[i])
```

Why `not_hold[i - 2]` for buying: if you sold yesterday, today is cooldown, so buying today must come from two days ago.

Return `not_hold[last]` because holding an unsold stock cannot increase final profit.

Time: `O(n)`. Space: `O(n)`, or `O(1)` with variables.

## Coin Change II

Unbounded knapsack counting combinations.

State:

```text
dp[i][a] = number of ways to make amount a using first i coins
```

Choices:

- skip current coin: `dp[i - 1][a]`
- include current coin: `dp[i][a - coin]`

Include uses the same row because the same coin can be used multiple times.

Base:

```text
dp[i][0] = 1
```

There is one way to make amount `0`: choose no coins.

Time: `O(c * amount)`, where `c = coins.len()`. Space: `O(c * amount)` full table.

## Target Sum

DP over reachable sums.

State:

```text
dp[i][sum] = number of ways to reach sum using first i numbers
```

Each number branches into two choices:

```rust
next[sum + num] += count;
next[sum - num] += count;
```

Base:

```text
dp[0][0] = 1
```

There is one way to reach sum `0` with zero numbers.

Use a `HashMap` because sums can be negative.

Optimized version keeps only the current map:

```rust
let mut next = HashMap::new();
for (sum, count) in dp {
    next[sum + num] += count;
    next[sum - num] += count;
}
dp = next;
```

Zeros are handled naturally: `+0` and `-0` update the same sum twice, doubling the count.

Time: `O(n * s)`, where `s` is the number of reachable sums. Worst case `s <= 2 * sum(nums) + 1`. Space: `O(s)` optimized, `O(n * s)` full table.
