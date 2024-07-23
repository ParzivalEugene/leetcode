#[cfg(test)]
mod test {
    use crate::solutions::p2610::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
            vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]]
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::find_matrix(vec![1, 2, 3, 4]),
            vec![vec![4, 3, 2, 1]]
        )
    }
}
