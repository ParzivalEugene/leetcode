#[cfg(test)]
mod test {
    use crate::solutions::p0136::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4)
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::single_number(vec![1]), 1)
    }
}
