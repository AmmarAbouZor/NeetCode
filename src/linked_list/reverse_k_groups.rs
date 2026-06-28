#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Reverse nodes in group of k.
//
// Before reversing a group, first check that at least k nodes remain. If fewer
// than k nodes remain, attach them unchanged.
//
// For each full group:
// 1. Detach exactly k nodes from the remaining list
// 2. Reverse the detached group
// 3. Append it to the result
//
// `tail` is mutable reference to the place where the next processed group should be attached.
//
// Time: O(n), because each node is checked, detached, reversed and appended in constant
// number of times.
// Space: O(1), reusing existing nodes.

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }

    let k = k as usize;
    let mut remaining = head;

    let mut result = None;
    let mut tail = &mut result;

    while has_k_nodes(remaining.as_deref(), k) {
        // Detach the next k nodes as one group.
        let mut group = remaining.take();
        let mut after_group = &mut group;

        for _ in 0..k {
            after_group = &mut after_group.as_mut().unwrap().next;
        }

        remaining = after_group.take();

        // Reverse the detached group and append it to the output.
        *tail = reverse_list(group);

        // Move tail to the end of the appended group.
        for _ in 0..k {
            tail = &mut tail.as_mut().unwrap().next;
        }
    }

    // Fewer than k nodes remain, so keep them unchanged.
    *tail = remaining;

    result
}

fn has_k_nodes(head: Option<&ListNode>, k: usize) -> bool {
    let mut current = head;
    for _ in 0..k {
        match current {
            Some(node) => current = node.next.as_deref(),
            None => return false,
        }
    }

    true
}

// Helper function taken from previous linked list problems.
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}
