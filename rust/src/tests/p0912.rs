#[cfg(test)]
mod test {
    use crate::solutions::p0912::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), [1, 2, 3, 5])
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            [0, 0, 1, 1, 2, 5]
        )
    }
}
