# 1D Dynamic Programming Notes

## General DP checklist

For DP, avoid jumping straight to optimized variables. First define the state.

1. Define what `dp[i]` means.
2. Write base cases from that definition.
3. Write the recurrence.
4. Confirm index meaning with a small example.
5. Optimize space only when each state depends on a small fixed number of previous states.

Common bug source: mixing two different states, for example:

- minimum cost to stand on stair `i`
- minimum cost to reach step `i`

Pick one and keep the base cases and recurrence consistent.

## Space optimization pattern

When `dp[i]` depends only on `dp[i - 1]` and `dp[i - 2]`:

```rust
let cur = recurrence(prev1, prev2);
prev2 = prev1;
prev1 = cur;
```

Use comments to name the variables:

```rust
let mut prev1 = ...; // dp[i - 1]
let mut prev2 = ...; // dp[i - 2]
```

## Climbing Stairs

State:

```text
dp[i] = number of ways to reach step i
```

Recurrence:

```text
dp[i] = dp[i - 1] + dp[i - 2]
```

The last move is either:

- 1 step from `i - 1`
- 2 steps from `i - 2`

Time: `O(n)`. Space: `O(n)` with array, `O(1)` with two variables.

## Min Cost Climbing Stairs

State:

```text
dp[i] = minimum cost to reach step i
```

Base cases:

```text
dp[0] = 0
dp[1] = 0
```

You can start at step `0` or step `1` for free. You pay when stepping from a stair.

Recurrence:

```text
dp[i] = min(dp[i - 1] + cost[i - 1],
            dp[i - 2] + cost[i - 2])
```

Time: `O(n)`. Space: `O(n)` with array, `O(1)` with two variables.

## House Robber

State:

```text
dp[i] = max money from houses 0..=i
```

At each house:

- skip it: `dp[i - 1]`
- rob it: `nums[i] + dp[i - 2]`

Recurrence:

```text
dp[i] = max(dp[i - 1], nums[i] + dp[i - 2])
```

Time: `O(n)`. Space: `O(n)` with array, `O(1)` with two variables.

## House Robber II

Houses are circular, so first and last cannot both be robbed.

Reduce to two House Robber I runs:

```rust
max(
    rob_linear(&nums[..n - 1]), // skip last
    rob_linear(&nums[1..]),     // skip first
)
```

Time: `O(n)`. Space: `O(1)` if helper uses two variables.

## Longest Palindromic Substring

This is often solved without DP using expand-around-center.

For every index, check:

- odd palindrome center: `(i, i)`
- even palindrome center: `(i, i + 1)`

Expand while characters match.

```rust
while r < s.len() && s[l] == s[r] {
    update_answer();
    let Some(next_l) = l.checked_sub(1) else {
        break;
    };
    l = next_l;
    r += 1;
}
```

`checked_sub` avoids `usize` underflow.

Time: `O(n^2)`. Space: `O(1)` excluding the returned string.

## Palindromic Substrings

Same expand-around-center pattern, but count every successful expansion.

```rust
count += 1;
```

Check both odd and even centers.

Time: `O(n^2)`. Space: `O(1)`.

## Decode Ways

At each position, there are at most two choices:

- decode current digit alone if it is `'1'..='9'`
- decode previous + current together if they form `10..=26`

Array state:

```text
dp[i] = number of ways to decode the first i characters
```

`dp` has length `n + 1` because `i` is prefix length, not string index.

Base cases:

```text
dp[0] = 1
dp[1] = 0 if s[0] == '0', else 1
```

`dp[0] = 1` means the empty prefix contributes one path for valid two-digit prefixes like `"10"` or `"12"`.

Transition:

```rust
if s[i - 1] != b'0' {
    dp[i] += dp[i - 1];
}

if (10..=26).contains(&two_digit) {
    dp[i] += dp[i - 2];
}
```

Time: `O(n)`. Space: `O(n)` with array, `O(1)` with two variables.

## Coin Change

State:

```text
dp[x] = minimum number of coins needed to make amount x
```

Base:

```text
dp[0] = 0
```

For each subtotal, try every coin:

```rust
if coin <= subtotal {
    dp[subtotal] = dp[subtotal].min(dp[subtotal - coin] + 1);
}
```

Initialize unreachable states with `amount + 1`, not `i32::MAX`, to avoid overflow when adding `1`.

Return `-1` if `dp[amount]` is still impossible.

Time: `O(amount * c)`, where `c = coins.len()`. Space: `O(amount)`.

## Longest Increasing Subsequence

Simple DP state:

```text
lis[i] = length of longest increasing subsequence starting at index i
```

Iterate from right to left so all `lis[j]` for `j > i` are already known.

```rust
for i in (0..n).rev() {
    let mut best = 1;
    for j in i + 1..n {
        if nums[i] < nums[j] {
            best = best.max(lis[j] + 1);
        }
    }
    lis[i] = best;
}
```

The answer is `max(lis)` because the subsequence can start at any index.

Time: `O(n^2)`. Space: `O(n)`.

Follow-up: patience sorting with binary search gives `O(n log n)`, but the `O(n^2)` DP is easier to explain first.
