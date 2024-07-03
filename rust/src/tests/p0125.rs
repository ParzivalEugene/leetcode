#[cfg(test)]
mod test {
    use crate::solutions::p0125::Solution;

    #[test]
    fn custom_case_1() {
        assert_eq!(
            Solution::is_palindrome(String::from("AmanaPLANaCANALPANAMA")),
            true
        );
    }

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::is_palindrome(String::from(" ")), true);
    }
}
