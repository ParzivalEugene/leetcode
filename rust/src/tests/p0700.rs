#[cfg(test)]
mod test {
    use crate::solutions::p0700::{Solution, TreeNode};

    #[test]
    fn leetcode_case_1() {
        let root = TreeNode::new_as_leaf(
            4,
            TreeNode::new_as_leaf(
                2,
                TreeNode::new_as_leaf(1, None, None),
                TreeNode::new_as_leaf(3, None, None),
            ),
            TreeNode::new_as_leaf(7, None, None),
        );
        let res = TreeNode::new_as_leaf(
            2,
            TreeNode::new_as_leaf(1, None, None),
            TreeNode::new_as_leaf(3, None, None),
        );
        assert_eq!(Solution::search_bst(root, 2), res)
    }

    #[test]
    fn leetcode_case_2() {
        let root = TreeNode::new_as_leaf(
            4,
            TreeNode::new_as_leaf(
                2,
                TreeNode::new_as_leaf(1, None, None),
                TreeNode::new_as_leaf(3, None, None),
            ),
            TreeNode::new_as_leaf(7, None, None),
        );
        let res = None;
        assert_eq!(Solution::search_bst(root, 5), res)
    }
}
