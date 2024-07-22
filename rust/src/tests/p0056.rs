#[cfg(test)]
mod test {
    use crate::solutions::p0056::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![5, 6]]),
            vec![vec![1, 4], vec![4, 6]]
        )
    }
}
