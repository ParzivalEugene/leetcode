pub struct Solution;

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

pub fn new_list(nodes: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;

    for &node in nodes.iter().rev() {
        let mut new_node = Box::new(ListNode::new(node));
        new_node.next = head;
        head = Some(new_node)
    }

    head
}

impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut i = &mut head;
        let mut j = &i.clone();
        while j.is_some() && j.as_ref().unwrap().next.is_some() {
            i = &mut i.as_mut().unwrap().next;
            j = &j.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        *i = i.as_mut().unwrap().next.take();

        head
    }
}
