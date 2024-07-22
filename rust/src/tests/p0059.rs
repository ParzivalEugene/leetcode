#[cfg(test)]
mod test {
    use crate::solutions::p0059::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]])
    }
}
