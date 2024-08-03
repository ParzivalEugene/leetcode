#[cfg(test)]
mod test {
    use crate::solutions::p3208::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3),
            3
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
            2
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4),
            0
        )
    }
}
