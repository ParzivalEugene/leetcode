#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

pub struct Solution;

impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut current = &mut head;
        while current.is_some() {
            if current.as_ref().unwrap().val == val {
                *current = current.as_mut().unwrap().next.take();
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }
        head
    }
}
