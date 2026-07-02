use std::{cmp::Reverse, collections::BinaryHeap};

// Two-heap simulation.
// Note: Check solution with sort below.
//
// `pending` stores tasks that have not become available yet, ordered by
// enqueue time. `ready` stores available tasks, ordered by processing time,
// then original index.
//
// At each step:
// - move every task with enqueue_time <= current time into `ready`
// - if a task is ready, run the shortest one
// - otherwise jump time to the next pending task's enqueue time
//
// This version is useful if tasks can be added dynamically while the scheduler
// is running, because future tasks can be pushed into `pending`.
//
// Time: O(n log n), because each task is pushed/popped from heaps.
// Space: O(n).

pub fn get_order_two_heaps(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let mut pending: BinaryHeap<Reverse<(u64, u64, usize)>> = tasks
        .into_iter()
        .enumerate()
        .map(|(idx, task)| {
            let enqueue = task[0] as u64;
            let process = task[1] as u64;
            Reverse((enqueue, process, idx))
        })
        .collect();

    let mut ready: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();

    let mut time: u64 = 0;
    let mut order = Vec::with_capacity(pending.len());

    while !pending.is_empty() || !ready.is_empty() {
        while let Some(&Reverse((enqueue, process, idx))) = pending.peek() {
            if enqueue > time {
                break;
            }

            pending.pop();
            ready.push(Reverse((process, idx)));
        }

        if let Some(Reverse((process, idx))) = ready.pop() {
            order.push(idx as i32);
            time += process;
        } else if let Some(&Reverse((enqueue, _, _))) = pending.peek() {
            time = enqueue;
        }
    }

    order
}

// Sort-upfront simulation.
//
// Since all tasks are known in advance, sort them by enqueue time and use one
// min-heap for tasks that are currently available.
//
// `idx` points to the next task that has not been added to the ready heap.
// The ready heap is ordered by processing time, then original index, matching
// the CPU tie-breaking rule.
//
// If no task is ready, jump time to the next task's enqueue time.
//
// This is the standard static-input solution. It is slightly simpler than the
// two-heap version, but less natural if tasks can arrive dynamically.
//
// Time: O(n log n), from sorting and heap operations.
// Space: O(n).
pub fn get_order_sort(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let mut tasks: Vec<(u64, u64, usize)> = tasks
        .into_iter()
        .enumerate()
        .map(|(idx, task)| {
            let enqueue = task[0] as u64;
            let process = task[1] as u64;
            (enqueue, process, idx)
        })
        .collect();

    tasks.sort_unstable();

    let mut task_heap: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();

    let mut idx = 0;
    let mut time = 0;
    let mut res = Vec::new();

    while !task_heap.is_empty() || idx < tasks.len() {
        while idx < tasks.len() && tasks[idx].0 <= time {
            let process = tasks[idx].1;
            let task_idx = tasks[idx].2;

            task_heap.push(Reverse((process, task_idx)));
            idx += 1;
        }

        if let Some(Reverse((process, task_idx))) = task_heap.pop() {
            time += process;
            res.push(task_idx as i32);
        } else {
            time = tasks[idx].0;
        }
    }

    res
}
