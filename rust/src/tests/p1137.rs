#[cfg(test)]
mod test {
    use crate::solutions::p1137::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::tribonacci(4), 4)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::tribonacci(25), 1389537)
    }
}
