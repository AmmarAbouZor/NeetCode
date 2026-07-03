use std::collections::VecDeque;

// Topological sort rows and columns independently.
//
// Row conditions decide the vertical order of numbers.
// Column conditions decide the horizontal order of numbers.
//
// If either graph has a cycle, no valid matrix exists.
//
// After both topological orders are built, each number gets one row position
// and one columns position. Place the number at the intersections.
//
// Time: O(K^2 + R + C), where R and C are row/column condition counts.
// Topo sorting costs O(K + R + C), and building the matrix costs O(K^2).
// Space: O(K^2 + R + C), including the output matrix and graph storage.

pub fn build_matrix(
    k: i32,
    row_conditions: Vec<Vec<i32>>,
    col_conditions: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let k = k as usize;

    let Some(row_order) = topo_sort(k, &row_conditions) else {
        return Vec::new();
    };
    let Some(col_order) = topo_sort(k, &col_conditions) else {
        return Vec::new();
    };

    let mut row_pos = vec![0; k + 1];
    for (idx, &row) in row_order.iter().enumerate() {
        row_pos[row] = idx;
    }

    let mut col_pos = vec![0; k + 1];
    for (idx, &col) in col_order.iter().enumerate() {
        col_pos[col] = idx;
    }

    let mut res = vec![vec![0; k]; k];

    for num in 1..=k {
        let row = row_pos[num];
        let col = col_pos[num];
        res[row][col] = num as i32;
    }

    res
}

// Kahn's topological sort for values 1..=k.
// Returns None if not all nodes can be processed, which means there is a cycle.
fn topo_sort(k: usize, conditions: &[Vec<i32>]) -> Option<Vec<usize>> {
    let mut graph = vec![Vec::new(); k + 1];
    let mut indegree = vec![0; k + 1];

    for cond in conditions {
        let from = cond[0] as usize;
        let to = cond[1] as usize;

        graph[from].push(to);
        indegree[to] += 1;
    }

    let mut queue: VecDeque<usize> = (1..=k).filter(|n| indegree[*n] == 0).collect();

    let mut res = Vec::with_capacity(k);

    while let Some(node) = queue.pop_front() {
        res.push(node);

        for &neighbor in &graph[node] {
            let degree = &mut indegree[neighbor];
            *degree -= 1;

            if *degree == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    // If fewer than k nodes were processed, the remaining nodes are in a cycle.
    (res.len() == k).then_some(res)
}
