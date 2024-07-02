#[cfg(test)]
mod test {
    use crate::solutions::p0055::Solution;

    #[test]
    fn leetcode_case_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(Solution::can_jump(nums), true);
    }

    #[test]
    fn leetcode_case_2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(Solution::can_jump(nums), false);
    }
}
