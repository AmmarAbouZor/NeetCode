// This question is about checking circular dependencies in directed graphs.
// Solution is with DFS or BFS. This one is DFS recursion (However stack is possible)

// Steps:
// * Build graph adjacent list `pre_map` (Usually map but here vector was enough)
// * Apply DFS on each node while keeping track on the current state.
// * Mark fully explored nodes as Resolved so future DFS calls return early.
// * If we visit node in same path twice then it's a circular reference

// Time: O(V + E), where V is num_courses and E is prerequisites.len().
// Each node is resolved once and each edge is checked once.
//
// Space: O(V + E), for the adjacency list, state array, recursion stack.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CourseState {
    Unvisited,
    Visiting,
    Resolved,
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;

    let mut pre_map = vec![Vec::new(); num_courses];

    for pre in prerequisites {
        pre_map[pre[0] as usize].push(pre[1] as usize);
    }

    let mut state = vec![CourseState::Unvisited; num_courses];

    (0..num_courses).all(|c| dfs(c, &pre_map, &mut state))
}

fn dfs(course: usize, pre_map: &[Vec<usize>], state: &mut [CourseState]) -> bool {
    match state[course] {
        CourseState::Visiting => return false,
        CourseState::Resolved => return true,
        CourseState::Unvisited => {}
    }

    state[course] = CourseState::Visiting;

    for child in &pre_map[course] {
        if !dfs(*child, pre_map, state) {
            return false;
        }
    }

    state[course] = CourseState::Resolved;

    true
}
