#[cfg(test)]
mod test {
    use crate::solutions::p0070::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
