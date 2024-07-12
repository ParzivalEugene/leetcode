#[cfg(test)]
mod test {
    use crate::solutions::p0724::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1)
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0)
    }
}
