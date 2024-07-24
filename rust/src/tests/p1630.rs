#[cfg(test)]
mod test {
    use crate::solutions::p1630::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![0, 0, 2],
                vec![2, 3, 5]
            ),
            [true, false, true]
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            ),
            [false, true, false, false, true, true]
        )
    }
}
