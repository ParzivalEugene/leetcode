#[cfg(test)]
mod test {
    use crate::solutions::p1679::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1)
    }
}
