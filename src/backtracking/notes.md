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
