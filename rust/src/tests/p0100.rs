#[cfg(test)]
mod test {
    use crate::solutions::p0100::{leaf, Solution};

    #[test]
    fn leetcode_case_1() {
        let left = leaf(1, leaf(2, None, None), leaf(3, None, None));
        let right = leaf(1, leaf(2, None, None), leaf(3, None, None));
        assert_eq!(Solution::is_same_tree(left, right), true);
    }

    #[test]
    fn leetcode_case_2() {
        let left = leaf(1, leaf(2, None, None), None);
        let right = leaf(1, None, leaf(2, None, None));
        assert_eq!(Solution::is_same_tree(left, right), false);
    }

    #[test]
    fn leetcode_case_3() {
        let left = leaf(1, leaf(2, None, None), leaf(1, None, None));
        let right = leaf(1, leaf(1, None, None), leaf(2, None, None));
        assert_eq!(Solution::is_same_tree(left, right), false);
    }
}
