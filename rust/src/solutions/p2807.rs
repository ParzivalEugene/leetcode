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
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let mut tail = &mut result;
        let mut prev = head.as_ref().unwrap().val;
        let mut iter = head.as_ref().unwrap().next.clone();
        tail = &mut tail.insert(head.unwrap()).next;

        while let Some(mut node) = iter {
            let current = node.val;
            tail = &mut tail
                .insert(Box::new(ListNode::new(Self::gcd(prev, current))))
                .next;
            iter = node.next.take();
            tail = &mut tail.insert(node).next;
            prev = current;
        }

        result
    }

    fn gcd(x: i32, y: i32) -> i32 {
        if y == 0 {
            x.abs()
        } else {
            Self::gcd(y, x % y)
        }
    }
}
