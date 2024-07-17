#[cfg(test)]
mod test {
    use crate::solutions::p0872::{Solution, TreeNode};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn leetcode_case_1() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode::new(
            3,
            TreeNode::new_as_leaf(
                5,
                TreeNode::new_as_leaf(6, None, None),
                TreeNode::new_as_leaf(
                    2,
                    TreeNode::new_as_leaf(7, None, None),
                    TreeNode::new_as_leaf(4, None, None),
                ),
            ),
            TreeNode::new_as_leaf(
                1,
                TreeNode::new_as_leaf(9, None, None),
                TreeNode::new_as_leaf(8, None, None),
            ),
        ))));
        let root2 = Some(Rc::new(RefCell::new(TreeNode::new(
            3,
            TreeNode::new_as_leaf(
                5,
                TreeNode::new_as_leaf(6, None, None),
                TreeNode::new_as_leaf(7, None, None),
            ),
            TreeNode::new_as_leaf(
                1,
                TreeNode::new_as_leaf(4, None, None),
                TreeNode::new_as_leaf(
                    2,
                    TreeNode::new_as_leaf(9, None, None),
                    TreeNode::new_as_leaf(8, None, None),
                ),
            ),
        ))));
        assert_eq!(Solution::leaf_similar(root1, root2), true);
    }

    #[test]
    fn leetcode_case_2() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode::new(
            1,
            TreeNode::new_as_leaf(2, None, None),
            TreeNode::new_as_leaf(3, None, None),
        ))));
        let root2 = Some(Rc::new(RefCell::new(TreeNode::new(
            1,
            TreeNode::new_as_leaf(3, None, None),
            TreeNode::new_as_leaf(2, None, None),
        ))));
        assert_eq!(Solution::leaf_similar(root1, root2), false);
    }
}
