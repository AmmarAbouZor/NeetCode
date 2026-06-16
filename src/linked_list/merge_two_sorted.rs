#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

// Time: O(n + m), where n is length of list1 and m is length of list2
// Space: O(1)

// Remember that tail is a reference so changing it directly will mean that
// It will refer to another point and will not move the current pointer to the new one.
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;

    let mut dummy = ListNode { val: 0, next: None };
    let mut tail = &mut dummy;

    while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
        if l1.val <= l2.val {
            tail.next = list1.take();
            // Move tail to next & Forward list1
            // This part feels awkward if we don't think about tail as pointer
            tail = tail.next.as_mut().unwrap();
            list1 = tail.next.take();
        } else {
            tail.next = list2.take();
            // Move tail to next & Forward list2
            tail = tail.next.as_mut().unwrap();
            list2 = tail.next.take();
        }
    }

    tail.next = list1.or(list2);

    dummy.next
}
