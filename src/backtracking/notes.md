For backtracking, using one consistent loop pattern will reduce mistakes under pressure.

Your core pattern can be:

```rust
fn dfs(start, cur, res) {
    record_if_needed();

    for i in start..choices.len() {
        if invalid_choice {
            continue_or_break;
        }

        choose;
        dfs(next_start, cur, res);
        undo;
    }
}
```

Then adapt:

### Subsets

```rust
res.push(cur.clone());

for i in start..nums.len() {
    cur.push(nums[i]);
    dfs(i + 1, cur, res);
    cur.pop();
}
```

### Subsets II

Same as subsets, but sort and skip duplicate sibling choices.

```rust
res.push(cur.clone());

for i in start..nums.len() {
    if i > start && nums[i] == nums[i - 1] {
        continue;
    }

    cur.push(nums[i]);
    dfs(i + 1, cur, res);
    cur.pop();
}
```

### Combination Sum I

```rust
if remaining == 0 {
    res.push(cur.clone());
    return;
}

for i in start..nums.len() {
    if nums[i] > remaining {
        break;
    }

    cur.push(nums[i]);
    dfs(i, remaining - nums[i], cur, res); // reuse allowed
    cur.pop();
}
```

### Combination Sum II

```rust
if remaining == 0 {
    res.push(cur.clone());
    return;
}

for i in start..nums.len() {
    // Skip duplicate sibling choices at same DFS level.
    if i > start && nums[i] == nums[i - 1] {
        continue;
    }

    if nums[i] > remaining {
        break;
    }

    cur.push(nums[i]);
    dfs(i + 1, remaining - nums[i], cur, res); // reuse not allowed
    cur.pop();
}
```

### Duplicate skipping

After sorting, skip duplicate sibling choices at the same DFS level:

```rust
if i > start && nums[i] == nums[i - 1] {
    continue;
}
```

This avoids duplicate results while still allowing duplicate values from deeper levels.

### Pruning with sorted positive numbers

If input is sorted and values are positive:

```rust
if nums[i] > remaining {
    break;
}
```

Without sorting, use `continue`, not `break`, because later values may be smaller.

### Permutations

Permutations are the exception because there is no `start`; every position can use any unused number.

```rust
for i in 0..nums.len() {
    if used[i] {
        continue;
    }

    used[i] = true;
    cur.push(nums[i]);

    dfs(nums, used, cur, res);

    cur.pop();
    used[i] = false;
}
```

### Generate Parentheses

Build only valid prefixes.

```rust
if open < n {
    cur.push('(');
    dfs(open + 1, close, cur, res);
    cur.pop();
}

if close < open {
    cur.push(')');
    dfs(open, close + 1, cur, res);
    cur.pop();
}
```

There are Catalan-number many valid outputs.

### Word Search

Grid backtracking pattern:

```rust
if out_of_bounds || visited[r][c] || board[r][c] != word[i] {
    return false;
}

visited[r][c] = true;
search neighbors;
visited[r][c] = false;
```

Time is exponential in word length because DFS branches in directions.

### Letter Combinations of Phone Number

This is not a `for i in start..nums.len()` problem. Each recursion level handles exactly one digit.

```rust
for ch in digit_chars(digits[idx]) {
    cur.push(ch);
    dfs(idx + 1, cur, res);
    cur.pop();
}
```

### N-Queens

Place one queen per row, so rows do not need tracking.

Track invalid future positions with:

```text
cols: used columns
main diagonal: r - c
anti diagonal: r + c
```

The set version is easiest to code in an interview because the diagonal formulas stay direct.

Optimized bool-array version:

```text
main_d_idx = r + n - 1 - c  // shifted r - c to avoid negative index
anti_d_idx = r + c
```

Backtrack by placing `Q`, marking column/diagonals, recursing to `r + 1`, then undoing.

Time: `O(n!)` with pruning. Space: `O(n^2)` for the board plus tracking state.

### Backtracking Big-O notes

- Subsets: `O(n * 2^n)`
- Permutations: `O(n * n!)`
- Combination Sum I: loose bound `O(2^(T/m))`, output can dominate
- Combination Sum II: `O(n * 2^n)`
- Generate Parentheses: `O(C_n * n)`
- Word Search: `O(R * C * 4 * 3^(L - 1))`
