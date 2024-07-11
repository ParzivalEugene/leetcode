#[cfg(test)]
mod test {
    use crate::solutions::p0334::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false)
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true)
    }
}
