#[cfg(test)]
mod test {
    use crate::solutions::p0020::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
