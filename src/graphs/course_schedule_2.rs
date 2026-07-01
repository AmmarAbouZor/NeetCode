// Similar to Course Schedule I, but here we need to return a valid ordering.
// NOTE: Please check solution below for Kahn's topological sort.

// Time: O(V + E), where V is num_courses and E is prerequisites.len().
// Each node is resolved once and each edge is checked once.
//
// Space: O(V + E), for the adjacency list, state array, recursion stack,
// and output array.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CourseState {
    Unvisited,
    Visiting,
    Resolved,
}

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;
    let mut pre_map = vec![Vec::new(); num_courses];

    for pair in prerequisites {
        pre_map[pair[0] as usize].push(pair[1] as usize);
    }

    let mut state = vec![CourseState::Unvisited; num_courses];

    let mut output = Vec::new();

    let all_resolved = (0..num_courses).all(|c| resolve_dfs(c, &pre_map, &mut state, &mut output));

    if all_resolved { output } else { Vec::new() }
}

fn resolve_dfs(
    course: usize,
    pre_map: &[Vec<usize>],
    state: &mut [CourseState],
    output: &mut Vec<i32>,
) -> bool {
    match state[course] {
        CourseState::Visiting => return false,
        CourseState::Resolved => return true,
        CourseState::Unvisited => {}
    }

    state[course] = CourseState::Visiting;

    for child in &pre_map[course] {
        if !resolve_dfs(*child, pre_map, state, output) {
            return false;
        }
    }

    state[course] = CourseState::Resolved;
    output.push(course as i32);

    true
}

// Kahn's BFS topological sort.
//
// Build edges prerequisite -> course. `indegree[c]` is the number of
// prerequisites course `c` still needs.
//
// Start with all courses that have indegree 0. When a course is taken, decrement
// the indegree of courses it unlocks. If a neighbor reaches 0, it is ready.
//
// If we cannot process all courses, a cycle exists, so return empty.
//
// Time: O(V + E)
// Space: O(V + E)
pub fn find_order_topo(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;

    let mut adj = vec![Vec::new(); num_courses];
    let mut indegree = vec![0; num_courses];

    for pre in prerequisites {
        let to = pre[0] as usize;
        let from = pre[1] as usize;

        adj[from].push(to);
        indegree[to] += 1;
    }

    let mut queue = std::collections::VecDeque::new();

    for course in 0..num_courses {
        if indegree[course] == 0 {
            queue.push_back(course);
        }
    }

    let mut res = Vec::with_capacity(num_courses);

    while let Some(course) = queue.pop_front() {
        res.push(course as i32);

        for &nei in &adj[course] {
            indegree[nei] -= 1;

            if indegree[nei] == 0 {
                queue.push_back(nei);
            }
        }
    }

    if res.len() == num_courses {
        res
    } else {
        Vec::new()
    }
}
