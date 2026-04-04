// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow.to_owned()
}

/// another version of solution.
/// much cleaner to read and reason with
pub fn middle_node_v2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while let (Some(_), Some(_)) = (fast, fast.and_then(|node| node.next.as_ref())) {
        fast = fast
            .and_then(|node| node.next.as_ref())
            .and_then(|node| node.next.as_ref());
        slow = slow.and_then(|node| node.next.as_ref());
    }
    slow.cloned()
}

#[cfg(test)]
mod tests {

    use crate::leetcode::p0876;

    #[test]
    fn middle_node() {}
}
