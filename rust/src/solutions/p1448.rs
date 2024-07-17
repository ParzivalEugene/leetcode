pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![(root, i32::MIN)];
        let mut counter = 0;

        while let Some(pair) = stack.pop() {
            if let (Some(node_rc), mut max) = pair {
                let mut node_r = node_rc.borrow_mut();
                if node_r.val >= max {
                    counter += 1;
                    max = node_r.val;
                }
                stack.push((node_r.left.take(), max));
                stack.push((node_r.right.take(), max));
            }
        }

        counter
    }
}
