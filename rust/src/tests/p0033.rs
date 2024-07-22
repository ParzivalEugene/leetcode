#[cfg(test)]
mod test {
    use crate::solutions::p0033::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
