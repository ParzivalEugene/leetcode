#[cfg(test)]
mod test {
    use crate::solutions::p0151::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::reverse_words(String::from("the sky is blue")),
            String::from("blue is sky the")
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::reverse_words(String::from("a good   example")),
            String::from("example good a")
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::reverse_words(String::from("EPY2giL")),
            String::from("EPY2giL")
        )
    }
}
