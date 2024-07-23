#[cfg(test)]
mod test {
    use crate::solutions::p3146::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::find_permutation_difference("abc".to_string(), "bac".to_string()),
            2
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string()),
            12
        );
    }
}
