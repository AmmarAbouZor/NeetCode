pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

// Do the addition simply by hand, digit by digit, carrying into the next position.
// The reversed order in the linked list is actually for our benefit.

// Calculating the sums of each number on its own will cause integer overflow, so process digits direclty.

// Time: O(max(l1, l2))
// Space: O(max(l1, l2))

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add(l1, l2, 0)
}

fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let val1 = l1.as_ref().map_or(0, |n| n.val);
    let val2 = l2.as_ref().map_or(0, |n| n.val);

    let sum = val1 + val2 + carry;
    let val = sum % 10;
    let carry = sum / 10;

    let next_node = add(l1.and_then(|n| n.next), l2.and_then(|n| n.next), carry);

    let node = ListNode {
        val,
        next: next_node,
    };

    Some(Box::new(node))
}
