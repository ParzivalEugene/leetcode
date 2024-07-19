#[cfg(test)]
mod test {
    use crate::solutions::p0226::{leaf, Solution};

    #[test]
    fn leetcode_case_1() {
        let left = leaf(2, leaf(1, None, None), leaf(3, None, None));
        let right = leaf(2, leaf(3, None, None), leaf(1, None, None));
        assert_eq!(Solution::invert_tree(left), right);
    }
}
