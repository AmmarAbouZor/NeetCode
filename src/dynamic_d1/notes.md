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

## Tribonacci Number

Same Fibonacci-style DP, but each state depends on the previous three states.

State:

```text
dp[i] = the i-th Tribonacci number
```

Base:

```text
dp[0] = 0
dp[1] = 1
dp[2] = 1
```

Recurrence:

```text
dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3]
```

Can be space-optimized with three variables:

```text
prev1 = dp[i - 1]
prev2 = dp[i - 2]
prev3 = dp[i - 3]
```

Time: `O(n)`. Space: `O(n)` with array, `O(1)` with three variables.

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

## Perfect Squares

Bottom-up DP, similar to Coin Change where the coins are square numbers.

State:

```text
dp[num] = minimum number of perfect squares needed to sum to num
```

Precompute all squares `<= n`, then for each `num`, try every square as the last picked value:

```text
dp[num] = min(dp[num], 1 + dp[num - square])
```

Base:

```text
dp[0] = 0
```

Use `n + 1` as infinity because the worst case is using `1^2` exactly `n` times.

Time: `O(n * sqrt(n))`. Space: `O(n + sqrt(n))`.

## Combination Sum IV

Bottom-up DP where order matters.

State:

```text
dp[total] = number of ordered combinations that sum to total
```

For each total, try every number as the last picked value:

```text
dp[total] += dp[total - num]
```

Base:

```text
dp[0] = 1
```

There is one way to build sum `0`: pick nothing. This lets a single number equal to `total` contribute one way.

Loop order matters. Iterating `total` outside and `nums` inside counts ordered sequences like `[1, 2]` and `[2, 1]` separately.

Time: `O(target * n)`, where `n = nums.len()`. Space: `O(target)`.

## Maximum Product Subarray

Kadane-style DP, but track both max and min product ending at current index.

State:

```text
cur_max = maximum product subarray ending here
cur_min = minimum product subarray ending here
```

Keep `cur_min` because a negative number can turn the smallest product into the largest product.

Candidate version:

```rust
candidates = [num, cur_max * num, cur_min * num]
cur_max = max(candidates)
cur_min = min(candidates)
```

Swap version:

```rust
if num < 0 {
    swap(cur_min, cur_max);
}
cur_max = max(num, cur_max * num);
cur_min = min(num, cur_min * num);
```

Zero resets the running product naturally because `num` itself is always a candidate.

Time: `O(n)`. Space: `O(1)`.

## Word Break

DP over prefixes.

State:

```text
dp[i] = true if s[0..i] can be segmented into dictionary words
```

For each end index `i`, try every split point `j`:

```rust
if dp[j] && words.contains(&s[j..i]) {
    dp[i] = true;
}
```

`dp[j]` means the prefix before `j` is valid. `s[j..i]` must be one dictionary word.

Base:

```text
dp[0] = true
```

The empty prefix is valid and lets the first word start at index `0`.

Let `n = s.len()` and `k = total chars in word_dict`.

Time: `O(k + n^3)` worst case, because there are `O(n^2)` splits and substring lookup hashes up to `O(n)` chars. Space: `O(k + n)`.

## Partition Equal Subset Sum

0/1 knapsack as a yes/no question.

If total sum is odd, equal partition is impossible. Otherwise target is:

```text
target = total / 2
```

State:

```text
dp[sum] = true if we can build sum using numbers processed so far
```

Base:

```text
dp[0] = true
```

For each `num`:

```rust
for sum in (num..=target).rev() {
    dp[sum] |= dp[sum - num];
}
```

Iterate backward so each number is used at most once.

Example if `num = 2` and we iterate forward:

- `dp[2]` becomes true from `dp[0]`
- then `dp[4]` becomes true from `dp[2]`

That incorrectly uses the same `2` twice in one pass. Backward iteration keeps `dp[sum - num]` from the previous state.

Time: `O(n * target)`, where `target = sum(nums) / 2`. Space: `O(target)`.

## Longest Increasing Subsequence

Default DP state:

```text
lis[i] = length of longest increasing subsequence ending at index i
```

Iterate left to right. For each `i`, check previous indices `j < i`.

If `nums[j] < nums[i]`, then `nums[i]` can extend the subsequence ending at `j`:

```rust
for i in 0..n {
    let mut best = 1;
    for j in 0..i {
        if nums[j] < nums[i] {
            best = best.max(lis[j] + 1);
        }
    }
    lis[i] = best;
}
```

The answer is `max(lis)` because the subsequence can end at any index.

Reverse direction also works with:

```text
lis[i] = length of longest increasing subsequence starting at index i
```

Then iterate from right to left and look at `j > i`.

Time: `O(n^2)`. Space: `O(n)`.

Follow-up: patience sorting with binary search gives `O(n log n)`, but the `O(n^2)` DP is easier to explain first.
