#[cfg(test)]
mod test {
    use crate::solutions::p0034::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1])
    }
}
