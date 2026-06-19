// Similar to Course Schedule I, but here we need to return a valid ordering.
// We also track resolved courses separately so each course is added to output
// exactly once.

// Time: O(V + E), where V is num_courses and E is prerequisites.len().
// Each node is resolved once and each edge is checked once.
//
// Space: O(V + E), for the adjacency list, visiting array, resolved set, recursion stack,
// and output array.

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;
    let mut pre_map = vec![Vec::new(); num_courses];

    for pair in prerequisites {
        pre_map[pair[0] as usize].push(pair[1] as usize);
    }

    let mut visiting = vec![false; num_courses];

    let mut resolved_set = vec![false; num_courses];
    let mut output = Vec::new();

    let all_resolved = (0..num_courses)
        .all(|c| resolve_dfs(c, &pre_map, &mut visiting, &mut resolved_set, &mut output));

    if all_resolved { output } else { Vec::new() }
}

fn resolve_dfs(
    course: usize,
    pre_map: &[Vec<usize>],
    visiting: &mut [bool],
    resolved_set: &mut [bool],
    output: &mut Vec<i32>,
) -> bool {
    if visiting[course] {
        return false;
    }

    if resolved_set[course] {
        return true;
    }

    visiting[course] = true;

    for child in &pre_map[course] {
        if !resolve_dfs(*child, pre_map, visiting, resolved_set, output) {
            return false;
        }
    }

    resolved_set[course] = true;
    visiting[course] = false;
    output.push(course as i32);

    // No need to clear out courses since we are keeping track on resolved
    // in separate set.
    //pre_map[course].clear();

    true
}
