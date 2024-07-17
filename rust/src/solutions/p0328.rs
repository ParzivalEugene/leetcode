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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut odd, mut even): (Vec<i32>, Vec<i32>) = (vec![], vec![]);
        let mut pool_switcher = true;
        let mut current_pointer = &head;
        while let Some(current) = current_pointer {
            match pool_switcher {
                true => odd.push(current.val),
                false => even.push(current.val),
            }
            pool_switcher = !pool_switcher;
            current_pointer = &current.next;
        }
        odd.extend(&even);
        new_list(odd)
    }
}
