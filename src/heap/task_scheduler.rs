// Standard solution for scheduler using heap for task counts and cooldown queue
// Each loop we take the task with most counts, run it and insert it to cooldown queue
// with the time for it to be active again.
// In the same loop, we check whether tasks are ready to run again and insert them to heap.
// We also skip empty loops by increasing cycle directly to the upcoming task in the queue in
// case the heap is empty.

// Time: O(N + T log A) ~= O(N + T)
// Space: O(A) ~= O(1)
// Where:
// - N = tasks.len()
// - T = final schedule length including idle cycles
// - A = number of unique task types, max 26

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    let mut counts = [0_usize; 26];
    for task in tasks {
        counts[(task as u8 - b'A') as usize] += 1;
    }

    let mut task_heap = std::collections::BinaryHeap::new();

    task_heap.extend(counts.into_iter().filter(|c| *c > 0));

    let mut cooldown_q = std::collections::VecDeque::new();

    let mut cycle = 0;
    while !task_heap.is_empty() || !cooldown_q.is_empty() {
        cycle += 1;

        if let Some(mut task_c) = task_heap.pop() {
            task_c -= 1;
            if task_c > 0 {
                cooldown_q.push_back((task_c, cycle + n));
            }
        } else {
            // Avoid empty cycles if heap is empty
            cycle = cooldown_q.front().unwrap().1;
        }

        while let Some(&(task_c, active_cycle)) = cooldown_q.front()
            && active_cycle <= cycle
        {
            let _ = cooldown_q.pop_front();
            task_heap.push(task_c);
        }
    }

    cycle
}

// Initial not good enough solution with
// Time: O(N + T * A log A) ~= O(N + T) worst case O(N * n)
// Space: O(A) ~= O(1)
// where:
// - N = tasks.len()
// - A = number of unique task types, max 26
// - T = final returned cycle_count, including idle cycles

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Task {
    count: usize,
    cycle: i32,
}

impl Task {
    fn new(count: usize, n: i32) -> Self {
        Self { count, cycle: -n }
    }
}

pub fn least_interval_no_queue(tasks: Vec<char>, n: i32) -> i32 {
    let mut counts = [0_usize; 26];

    // O(n)
    for ch in tasks {
        counts[(ch as u8 - b'A') as usize] += 1;
    }

    let mut tasks_heap = std::collections::BinaryHeap::with_capacity(26);

    tasks_heap.extend(
        counts
            .into_iter()
            .filter(|c| *c > 0)
            .map(|c| Task::new(c, n)),
    );

    let mut cycle_count = 0;

    let mut task_cache = Vec::new();

    while !tasks_heap.is_empty() {
        cycle_count += 1;

        while let Some(mut task) = tasks_heap.pop() {
            if cycle_count - task.cycle > n {
                task.count -= 1;
                task.cycle = cycle_count;
                if task.count > 0 {
                    task_cache.push(task);
                }
                break;
            }
            task_cache.push(task);
        }

        tasks_heap.extend(task_cache.drain(..))
    }

    cycle_count
}
