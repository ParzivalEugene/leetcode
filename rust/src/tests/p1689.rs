#[cfg(test)]
mod test {
    use crate::solutions::p1689::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::min_partitions("32".to_string()), 3);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::min_partitions("82734".to_string()), 8);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::min_partitions("27346209830709182346".to_string()),
            9
        );
    }
}
