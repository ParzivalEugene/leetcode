#[cfg(test)]
mod test {
    use crate::solutions::p0392::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::is_subsequence(String::from("abc"), String::from("ahbgdc")),
            true
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::is_subsequence(String::from("axc"), String::from("ahbgdc")),
            false
        );
    }
}
