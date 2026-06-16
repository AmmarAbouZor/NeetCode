#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

// The algorithm is to split the lists in half, reverse the second half
// then merge the two lists together.
//
// Reversing the second half is usually done using slow and fast pointers, but this isn't possible
// in rust because we need mutable reference for slow pointer while running the fast one. (Technically
// it's possible but it's really awkward)
// The traverse the list to get it's count. then traverse to the start of second half and reverse it
// This will include extra traverse but will still O(N).

// Time: O(N)
// Space: O(1)

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let len = get_len_iter(head);

    // Check if the list is long enough for reordering
    if len < 3 {
        return;
    }

    // We need to go to the last element of first and take its next as first second
    // so we own the value.
    // let first_len = (len + 1) / 2;
    let first_len = len.div_ceil(2);
    let mut first_tail = head.as_mut();
    for _ in 1..first_len {
        first_tail = first_tail.unwrap().next.as_mut();
    }

    // Reverse second half
    let mut second = first_tail.unwrap().next.take();
    second = reverse_list(second);

    // Merge two lists together.
    let mut first = head.as_mut();
    while let Some(mut second_node) = second.take() {
        let first_node = first.unwrap();
        let first_next = first_node.next.take();

        second = second_node.next.take();
        second_node.next = first_next;

        first_node.next = Some(second_node);
        first = first_node.next.as_mut().unwrap().next.as_mut();
    }
}

/// Gets the total length of linked list using iter::successors
fn get_len_iter(head: &Option<Box<ListNode>>) -> usize {
    std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count()
}

// Here for comparison with successors pattern
#[allow(unused)]
fn get_len_loop(head: &Option<Box<ListNode>>) -> usize {
    let mut len = 0;
    let mut current = head.as_ref();

    while let Some(node) = current {
        len += 1;
        current = node.next.as_ref();
    }
    len
}

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}
