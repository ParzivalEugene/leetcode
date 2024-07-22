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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut iter = head;

        while let Some(mut node) = iter {
            let mut sum = node.val;

            loop {
                node = node.next.unwrap();

                if node.val == 0 {
                    iter = node.next.take();
                    node.val = sum;
                    tail = &mut tail.insert(node).next;

                    break;
                }

                sum += node.val;
            }
        }

        result
    }
}
