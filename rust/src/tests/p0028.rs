#[cfg(test)]
mod test {
    use crate::solutions::p0028::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::str_str(String::from("sadbutsad"), String::from("sad")),
            0
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::str_str(String::from("leetcode"), String::from("leeto")),
            -1
        );
    }
}
