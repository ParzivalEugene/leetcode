#[cfg(test)]
mod test {
    use crate::solutions::p0169::Solution;

    #[test]
    fn leetcode_case_1() {
        let nums = vec![3, 2, 3];
        assert_eq!(Solution::majority_element(nums), 3);
    }

    #[test]
    fn leetcode_case_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element(nums), 2);
    }
}
