use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        TreeNode {
            val,
            left: match left {
                Some(node) => Some(Rc::new(RefCell::new(node))),
                None => None,
            },
            right: match right {
                Some(node) => Some(Rc::new(RefCell::new(node))),
                None => None,
            },
        }
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(branch) => {
                return 1 + std::cmp::max(
                    Self::max_depth(branch.borrow().left.clone()),
                    Self::max_depth(branch.borrow().right.clone()),
                )
            }
            None => 0,
        }
    }
}
