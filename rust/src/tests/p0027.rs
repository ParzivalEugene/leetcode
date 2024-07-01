#[cfg(test)]
mod test {
    use crate::solutions::p0027::Solution;

    #[test]
    fn leetcode_case_1() {
        let (mut nums, val) = (vec![3, 2, 2, 3], 3);
        assert_eq!(Solution::remove_element(&mut nums, val), 2);
    }

    #[test]
    fn leetcode_case_2() {
        let (mut nums, val) = (vec![0, 1, 2, 2, 3, 0, 4, 2], 2);
        assert_eq!(Solution::remove_element(&mut nums, val), 5);
    }
}
