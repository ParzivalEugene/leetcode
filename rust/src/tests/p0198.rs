#[cfg(test)]
mod test {
    use crate::solutions::p0198::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
