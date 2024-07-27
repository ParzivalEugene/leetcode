#[cfg(test)]
mod test {
    use crate::solutions::p1365::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        )
    }
}
