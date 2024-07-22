#[cfg(test)]
mod test {
    use crate::solutions::p3110::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::score_of_string("hello".to_string()), 13);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::score_of_string("zaz".to_string()), 50);
    }
}
