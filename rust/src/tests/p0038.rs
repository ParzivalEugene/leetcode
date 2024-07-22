#[cfg(test)]
mod test {
    use crate::solutions::p0038::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
    }
}
