#[cfg(test)]
mod test {
    use crate::solutions::p0771::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
            3
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
            0
        )
    }
}
