#[cfg(test)]
mod test {
    use crate::solutions::p0217::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false)
    }
}
