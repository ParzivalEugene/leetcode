#[cfg(test)]
mod test {
    use crate::solutions::p1508::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4), 6);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 10), 50);
    }

    #[test]
    fn leetcode_case_53() {
        assert_eq!(
            Solution::range_sum(vec![100; 1000], 1000, 1, 500500),
            716699888
        );
    }
}
