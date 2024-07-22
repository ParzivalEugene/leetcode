#[cfg(test)]
mod test {
    use crate::solutions::p2769::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::the_maximum_achievable_x(4, 1), 6)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::the_maximum_achievable_x(3, 2), 7)
    }
}
