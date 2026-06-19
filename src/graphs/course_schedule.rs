// This question is about checking circular dependencies in directed graphs.
// Solution is with DFS or BFS. This one is DFS recursion (However stack is possible)

// Steps:
// * Build graph adjacent list `pre_map` (Usually map but here vector was enough)
// * Apply DFS on each node while keeping track on the current visited nodes
// * Clear out resolved nodes
// * If we visit node in same path twice then it's a circular reference

// Time: O(V + E), where V is num_courses and E is prerequisites.len().
// Each node is resolved once and each edge is checked once.
//
// Space: O(V + E), for the adjacency list, visiting array, recursion stack,

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;

    let mut pre_map = vec![Vec::new(); num_courses];

    for pre in prerequisites {
        pre_map[pre[0] as usize].push(pre[1] as usize);
    }

    let mut visiting = vec![false; num_courses];

    (0..num_courses).all(|c| dfs(c, &mut pre_map, &mut visiting))
}

fn dfs(course: usize, pre_map: &mut Vec<Vec<usize>>, visiting: &mut Vec<bool>) -> bool {
    if visiting[course] {
        return false;
    }

    if pre_map[course].is_empty() {
        return true;
    }

    visiting[course] = true;

    let children = pre_map[course].clone();
    for child in children {
        if !dfs(child, pre_map, visiting) {
            return false;
        }
    }

    pre_map[course].clear();
    visiting[course] = false;

    true
}
