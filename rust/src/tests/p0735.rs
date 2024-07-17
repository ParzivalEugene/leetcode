#[cfg(test)]
mod test {
    use crate::solutions::p0735::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }
}
