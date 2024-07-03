#[cfg(test)]
mod test {
    use crate::solutions::p1550::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
            true
        )
    }
}
