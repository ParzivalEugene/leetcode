#[cfg(test)]
mod test {
    use crate::solutions::p0104::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode::new(
                3,
                Some(TreeNode::new(9, None, None)),
                Some(TreeNode::new(
                    20,
                    Some(TreeNode::new(15, None, None)),
                    Some(TreeNode::new(7, None, None))
                ))
            ))))),
            3
        )
    }
}
