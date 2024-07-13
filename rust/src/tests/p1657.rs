#[cfg(test)]
mod test {
    use crate::solutions::p1657::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::close_strings(String::from("abc"), String::from("bca")),
            true
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::close_strings(String::from("a"), String::from("aa")),
            false
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::close_strings(String::from("cabbba"), String::from("abbccc")),
            true
        )
    }
}
