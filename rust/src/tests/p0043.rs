#[cfg(test)]
mod test {
    use crate::solutions::p0043::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::multiply("2".to_owned(), "3".to_owned()), "6")
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::multiply("123".to_owned(), "456".to_owned()),
            "56088"
        )
    }
}
