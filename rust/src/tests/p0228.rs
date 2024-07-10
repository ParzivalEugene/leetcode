#[cfg(test)]
mod test {
    use crate::solutions::p0228::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        )
    }
}
