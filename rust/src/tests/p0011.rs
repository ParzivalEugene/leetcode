#[cfg(test)]
mod test {
    use crate::solutions::p0011::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
