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

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = &mut list1;
        while list2.is_some() {
            if current.is_none() || list2.as_ref()?.val < current.as_ref()?.val {
                std::mem::swap(current, &mut list2);
            }
            current = &mut current.as_mut()?.next;
        }

        list1
    }
}
