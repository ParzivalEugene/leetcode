#[cfg(test)]
mod test {
    use crate::solutions::p1460::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]),
            true
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::can_be_equal(vec![7], vec![7]), true);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::can_be_equal(vec![1, 12], vec![12, 1]), true);
    }
}
