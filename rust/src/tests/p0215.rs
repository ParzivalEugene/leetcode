#[cfg(test)]
mod test {
    use crate::solutions::p0215::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        )
    }
}
