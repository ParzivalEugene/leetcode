#[cfg(test)]
mod test {
    use crate::solutions::p0202::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::is_happy(19), true)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::is_happy(2), false)
    }
}
