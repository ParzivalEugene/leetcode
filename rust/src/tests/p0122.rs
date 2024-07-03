#[cfg(test)]
mod test {
    use crate::solutions::p0122::Solution;

    #[test]
    fn leetcode_case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices), 7);
    }

    #[test]
    fn leetcode_case_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
    }

    #[test]
    fn leetcode_case_3() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(prices), 4);
    }
}