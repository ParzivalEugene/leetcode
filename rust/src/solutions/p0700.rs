pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }

    pub fn new_as_leaf(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val, left, right))))
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node_rc) = root.clone() {
            let node = node_rc.borrow();
            match node.val.cmp(&val) {
                std::cmp::Ordering::Less if node.right.is_some() => root = node.right.clone(),
                std::cmp::Ordering::Equal => return root,
                std::cmp::Ordering::Greater if node.left.is_some() => root = node.left.clone(),
                _ => break,
            }
        }
        None
    }
}
