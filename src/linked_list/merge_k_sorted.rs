pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Divide-and-conquer merge for k sorted linked lists.
//
// `merge_two_lists` is the conquer step. Each round merges lists in pairs and stores the
// results in `next_round`, so a merged list is not merged again until the next round.
//
// Each node participates in one merge per round, and there are O(log K) rounds.
//
// Time: O(N logK), where N is the total number of nodes and K is lists.len().
// Space: O(K) for round vectors, the list nodes are reused.
pub fn merge_k_lists_div_conq(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;

    while lists.len() > 1 {
        let mut next_round = Vec::with_capacity(lists.len().div_ceil(2));

        while let Some(first) = lists.pop() {
            if let Some(second) = lists.pop() {
                let merged = merge_two_lists(first, second);
                next_round.push(merged);
            } else {
                next_round.push(first);
            }
        }

        lists = next_round
    }

    lists.into_iter().next()?
}

// This will be the conquer step.
fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: None };

    let mut tail = &mut dummy;

    while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
        if l1.val <= l2.val {
            tail.next = list1.take();
            tail = tail.next.as_mut().unwrap();
            list1 = tail.next.take();
        } else {
            tail.next = list2.take();
            tail = tail.next.as_mut().unwrap();
            list2 = tail.next.take();
        }
    }

    tail.next = list1.or(list2);

    dummy.next
}

// Sequential version for comparison.
// Correct, but not balanced: merged nodes may be processed again immediately.
// Wors-case time: O(N * K).
pub fn merge_k_lists_sequental(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;

    while lists.len() > 1 {
        let first = lists.pop().unwrap();
        let second = lists.pop().unwrap();
        let merged = merge_two_lists(first, second);
        lists.push(merged);
    }

    lists.into_iter().next()?
}
