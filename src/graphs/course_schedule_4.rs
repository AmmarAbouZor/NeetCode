// Topological sort + prerequisite propagation.
//
// Build edges prerequisite -> course. During Kahn's topological sort, each
// parent is processed before its children, so `deps[parent]` is complete before
// we merge it into each child.
//
// For every edge parent -> child:
// - parent is a direct prerequisite of child
// - everything in deps[parent] is alos an indirect prerequisite of child
//
// After pre-processing, each query [u, v] is answered by checking whether
// deps[v] contains u.
//
// Time: O(E * V + Q), because each edge may merge up to O(V) prerequisites.
// Space: O(V^2 + E + Q), or O(V^2 + E) excluding the output.

pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
) -> Vec<bool> {
    let num_courses = num_courses as usize;

    let mut adj = vec![Vec::new(); num_courses];
    let mut indegree = vec![0; num_courses];
    let mut deps = vec![std::collections::HashSet::new(); num_courses];

    for pre in prerequisites {
        let from = pre[0] as usize;
        let to = pre[1] as usize;

        adj[from].push(to);
        indegree[to] += 1;
    }

    let mut queue = std::collections::VecDeque::new();

    for course in 0..num_courses {
        if indegree[course] == 0 {
            queue.push_back(course);
        }
    }

    while let Some(parent) = queue.pop_front() {
        for &child in &adj[parent] {
            deps[child].insert(parent);

            // O(V)
            let parent_deps = deps[parent].clone();
            deps[child].extend(parent_deps);

            indegree[child] -= 1;

            if indegree[child] == 0 {
                queue.push_back(child);
            }
        }
    }

    queries
        .into_iter()
        .map(|q| deps[q[1] as usize].contains(&(q[0] as usize)))
        .collect()
}
