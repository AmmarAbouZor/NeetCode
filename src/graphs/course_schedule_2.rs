// Similar to Course Schedule I, but here we need to return a valid ordering.

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
