# Graph Notes

## General patterns

Graphs often appear as:

- grid cells with 4-direction neighbors
- adjacency lists from edge lists
- implicit state transitions
- dependency graphs

Pick traversal based on what the problem asks:

- DFS/BFS for connected components and reachability
- multi-source BFS for nearest-distance problems
- DFS state coloring or Kahn's algorithm for course scheduling
- Union Find for undirected connectivity and cycle detection

## Grid traversal

Use the grid as an implicit graph. Each cell is a node and neighbors are usually up/down/left/right.

```rust
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

for (dr, dc) in DIRS {
    let Some(r) = cur_r.checked_add_signed(dr) else {
        continue;
    };
    let Some(c) = cur_c.checked_add_signed(dc) else {
        continue;
    };

    if r >= height || c >= width {
        continue;
    }
}
```

`checked_add_signed` avoids `usize` underflow. It is clearer than wrapping arithmetic for interviews.

Mark cells visited when pushing/enqueuing, not when popping, to avoid adding the same cell multiple times.

## Number of Islands

Scan every cell. When an unvisited land cell is found, it starts a new island. BFS/DFS marks the whole connected component.

```rust
if grid[r][c] == '1' && !visited[r][c] {
    islands += 1;
    bfs_or_dfs(r, c);
}
```

Time: `O(n * m)`. Space: `O(n * m)` for visited and queue/stack.

## Max Area of Island

Same connected-component pattern, but count cells in each component and keep the max.

```rust
area += 1;
max_area = max_area.max(area);
```

Time: `O(n * m)`. Space: `O(n * m)`.

## Multi-source BFS

Use when each cell needs distance to the nearest source.

Start by pushing all sources into the queue, then expand level by level.

```rust
for each cell {
    if source {
        queue.push_back(cell);
    }
}
```

The first time a cell is reached is the shortest distance to any source.

## Islands and Treasure

Start BFS from all treasure cells `0`, not from each land cell.

Only update unreached land cells:

```rust
if grid[r][c] == i32::MAX {
    grid[r][c] = grid[cur_r][cur_c] + 1;
    queue.push_back((r, c));
}
```

Time: `O(n * m)`. Space: `O(n * m)` for the queue.

## Rotting Oranges

Multi-source BFS from all initially rotten oranges.

Each BFS level is one minute. Track fresh count to detect impossible cases.

```rust
while fresh_count > 0 && !queue.is_empty() {
    let level_len = queue.len();
    for _ in 0..level_len {
        // rot adjacent fresh oranges
    }
    minutes += 1;
}
```

Return `-1` if fresh oranges remain.

## Pacific Atlantic Water Flow

Reverse the flow.

Instead of asking whether each cell can reach both oceans, start from each ocean border and climb inward to cells with height `>= current`.

```rust
if heights[next] >= heights[cur] {
    visit next;
}
```

A cell in both visited sets can reach both oceans.

Time: `O(n * m)` because each cell is visited at most once per ocean. Space: `O(n * m)`.

## Surrounded Regions

Border-connected `O` cells are safe. Start DFS/BFS from the borders and mark all reachable `O`s.

Final pass:

- unmarked `O` becomes `X`
- marked `O` stays `O`

Can avoid extra visited space by temporarily marking safe cells in the board.

Time: `O(n * m)`. Space: `O(n * m)`.

## Clone Graph

Graphs can contain cycles and shared neighbors. Keep a map from original node identity to cloned node.

For LeetCode, node values are unique, so `val` can be used as the key.

```rust
if let Some(clone) = map.get(&val) {
    return Rc::clone(clone);
}

let cloned = Rc::new(RefCell::new(Node::new(val)));
map.insert(val, Rc::clone(&cloned));
```

Insert the clone before cloning neighbors so cycles terminate.

Time: `O(V + E)`. Space: `O(V)` excluding output.

## Course Schedule

Directed graph cycle detection.

Use DFS state coloring:

```rust
enum State {
    Unvisited,
    Visiting,
    Resolved,
}
```

Rules:

- `Visiting` means node is in the current DFS path
- seeing `Visiting` again means cycle
- `Resolved` means already proven safe

Time: `O(V + E)`. Space: `O(V + E)`.

## Course Schedule II

Same cycle detection, but push course after resolving prerequisites.

If graph is `course -> prerequisites`, postorder directly gives a valid order:

```rust
for prereq in graph[course] {
    dfs(prereq);
}
output.push(course);
```

If graph is `prerequisite -> unlocked courses`, output handling is different, often reversed or handled with Kahn's algorithm.

## Valid Tree

An undirected graph is a tree if:

- it has no cycles
- it is connected

DFS with parent tracking:

```rust
if visited[node] {
    return false;
}

for neighbor in adj[node] {
    if Some(neighbor) == parent {
        continue;
    }
    dfs(neighbor, Some(node));
}
```

After traversal, `visited.len() == n` ensures connectedness.

Fast check: a tree with `n` nodes must have exactly `n - 1` edges.

## Connected Components

Scan every node. Each unvisited node starts a new component.

```rust
for node in 0..n {
    if !visited[node] {
        components += 1;
        dfs(node);
    }
}
```

Time: `O(V + E)`. Space: `O(V + E)` for adjacency list and traversal state.

## Union Find / DSU

Useful for undirected connectivity and cycle detection.

Core operations:

- `find`: return component root
- `union`: merge two components if different

Path compression:

```rust
while cur != parent[cur] {
    parent[cur] = parent[parent[cur]];
    cur = parent[cur];
}
```

Union by size:

```rust
if size[root1] > size[root2] {
    parent[root2] = root1;
    size[root1] += size[root2];
} else {
    parent[root1] = root2;
    size[root2] += size[root1];
}
```

Only root sizes are meaningful. No need to reset the size of a node after it stops being a root.

Time: `O((V + E) * α(V))` where alpha is the inverse Ackermann function, effectively near linear. 
Space: `O(V)`.

## Components with Union Find

Start with `components = n`. Every successful union reduces components by one.

```rust
if dsu.union(a, b) {
    components -= 1;
}
```

## Redundant Connection

Original graph is a tree plus one extra edge. Process edges with Union Find.

If union fails, both endpoints are already connected, so this edge creates the cycle.

```rust
if !dsu.union(u, v) {
    return edge;
}
```

Time: near `O(E)`. Space: `O(V)`.
