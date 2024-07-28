#[cfg(test)]
mod test {
    use crate::solutions::p1395::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
    }
}
