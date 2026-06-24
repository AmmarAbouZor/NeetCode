pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Get the full length of the linked list in the first run, then iterate to reach the
// node to remove and replace it with the next node attached to it.

// Time: O(n), one pass to count and one pass to reach the node before removal.
// Space: O(1)

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let len = std::iter::successors(head.as_deref(), |n| n.next.as_deref()).count();

    let target = len
        .checked_sub(n as usize)
        .expect("Constraints state that len > n");

    let mut dummy = ListNode { val: 0, next: head };

    let mut curr = &mut dummy;

    for _ in 0..target {
        curr = curr.next.as_mut().unwrap();
    }
    // Skip the target node by linking the previous node to target.next.
    let next = curr.next.as_mut().unwrap().next.take();

    curr.next = next;

    dummy.next
}
