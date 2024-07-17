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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut current_reference = &head;
        let mut arr: Vec<i32> = vec![];
        while let Some(current) = current_reference {
            arr.push(current.val);
            current_reference = &current.next;
        }
        let len = arr.len();
        let (mut local, mut max) = (0, 0);
        for i in 0..len / 2 {
            local = arr[i] + arr[len - i - 1];
            if local > max {
                max = local;
            }
        }
        max
    }
}
