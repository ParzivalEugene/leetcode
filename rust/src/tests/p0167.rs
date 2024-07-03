#[cfg(test)]
mod test {
    use crate::solutions::p0167::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
