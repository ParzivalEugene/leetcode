#[cfg(test)]
mod test {
    use crate::solutions::p0485::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        )
    }
}
