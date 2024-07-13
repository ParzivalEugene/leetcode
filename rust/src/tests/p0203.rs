#[cfg(test)]
mod test {
    use crate::solutions::p0203::{ListNode, Solution};

    #[test]
    fn leetcode_case_1() {
        let list = ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(
                    6,
                    Some(Box::new(ListNode::new(
                        3,
                        Some(Box::new(ListNode::new(
                            4,
                            Some(Box::new(ListNode::new(
                                5,
                                Some(Box::new(ListNode::new(6, None))),
                            ))),
                        ))),
                    ))),
                ))),
            ))),
        );
        let res = ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(
                    3,
                    Some(Box::new(ListNode::new(
                        4,
                        Some(Box::new(ListNode::new(5, None))),
                    ))),
                ))),
            ))),
        );
        assert_eq!(
            Solution::remove_elements(Some(Box::new(list)), 6),
            Some(Box::new(res))
        )
    }
}
