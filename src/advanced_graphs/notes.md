# Advanced Graphs Notes

## General patterns

Advanced graph problems usually need one of these:

- Dijkstra for shortest paths with positive weights
- Bellman-Ford style DP when edge count/stops are constrained
- Prim/Kruskal for minimum spanning tree

Always identify:

- directed or undirected
- weighted or unweighted
- positive weights or possible negatives
- whether the state is only `node`, or needs extra info like `stops`

## Reconstruct Itinerary

Eulerian path in a directed graph. Each ticket is one directed edge and must be used exactly once.

Use Hierholzer's algorithm starting from `JFK`.

Sort each adjacency list in descending lexical order so `pop()` returns the smallest destination.

DFS consumes outgoing edges first, then pushes the airport in postorder:

```text
while next = graph[node].pop():
    dfs(next)
res.push(node)
```

Postorder builds the itinerary in reverse, so reverse `res` at the end.

If no solution is guaranteed, validate `res.len() == tickets.len() + 1` to ensure all edges were used.

Time: `O(E log E)` for sorting plus `O(E)` DFS. Space: `O(E)`.

## Network Delay Time

Shortest paths from one source in a directed weighted graph with positive weights.

Use Dijkstra.

State:

```text
dist[node] = shortest known time from start to node
```

Use a min-heap with `Reverse`:

```rust
heap.push(Reverse((time, node)));
```

Skip stale heap entries:

```rust
if time > dist[node] {
    continue;
}
```

After Dijkstra, answer is the maximum distance among all nodes. If any node is still unreachable, return `-1`.

Node labels are `1..=n`, so use vectors of length `n + 1` and skip index `0`.

Time: `O((V + E) log V)`, often written as `O(E log V)`. Space: `O(V + E)`.

## Swim in Rising Water

Dijkstra / minimum bottleneck path.

Path cost is the maximum elevation seen on the path, not the sum of edge costs.

Transition:

```text
next_time = max(current_time, grid[next_r][next_c])
```

Use a min-heap ordered by required time. The first time the bottom-right cell is popped, that time is optimal.

Time: `O(n^2 log n)` for an `n x n` grid. Space: `O(n^2)`.

## Min Cost to Connect Points

Minimum spanning tree problem.

Every point can connect to every other point using Manhattan distance, so the graph is complete.

```text
distance = |x1 - x2| + |y1 - y2|
```

For this complete graph, array-based Prim is better than heap Prim.

State:

```text
min_dist[i] = cheapest known cost to connect point i to the current MST
```

Each step:

1. choose the unvisited point with smallest `min_dist`
2. add it to the MST
3. update `min_dist` for every other unvisited point

```rust
min_dist[i] = min(min_dist[i], distance(point, i));
```

Time: `O(n^2)`. Space: `O(n)`.

Heap Prim also works, but because the graph is complete it may push `O(n^2)` edges:

```text
Time: O(n^2 log n)
Space: O(n^2)
```

Use the no-heap version as the main interview answer for this problem.

## Cheapest Flights Within K Stops

This looks like shortest path, but normal Dijkstra state is not enough.

A path to the same city with higher cost but fewer stops can still be better later. The state depends on both city and flights used.

A clean solution is Bellman-Ford style DP with a stop limit.

`k` stops means at most `k + 1` flights/edges.

State:

```text
dist[v] = cheapest cost to reach city v using at most the flights allowed so far
```

Each round allows one more flight.

Important: clone `dist` each round:

```rust
let mut next = dist.clone();
```

Read from previous round `dist`, write into current round `next`.

This prevents one round from chaining multiple flights and violating the stop limit.

Example with `k = 0`:

```text
0 -> 1 -> 2
```

Only one direct flight is allowed. In-place updates could update `1` and then `2` in the same round, incorrectly using two flights.

Relax every flight:

```rust
if dist[u] != inf {
    next[v] = next[v].min(dist[u] + price);
}
```

Use `i32::MAX / 2` as infinity to avoid overflow when adding flight cost.

Time: `O((k + 1) * E)`. Space: `O(V)`.
