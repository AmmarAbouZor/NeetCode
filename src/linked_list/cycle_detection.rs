#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: *mut ListNode,
}

// Time: O(N) from traversing the list
// Space: O(1)

/// # Safety
///
/// `head` must be null or point to live valid `ListNode`s. Every reachable `next`
/// pointer must also be null or point to live valid `ListNode`.
pub unsafe fn has_cycle(head: *mut ListNode) -> bool {
    let mut slow = head;
    let mut fast = head;

    loop {
        if slow.is_null() || fast.is_null() {
            return false;
        }

        let fast_next = unsafe { (*fast).next };

        if fast_next.is_null() {
            return false;
        }

        fast = unsafe { (*fast_next).next };
        slow = unsafe { (*slow).next };

        if fast == slow {
            return true;
        }
    }
}
