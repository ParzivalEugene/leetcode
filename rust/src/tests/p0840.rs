#[cfg(test)]
mod test {
    use crate::solutions::p0840::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::num_magic_squares_inside(vec![
                vec![4, 3, 8, 4],
                vec![9, 5, 1, 9],
                vec![2, 7, 6, 2]
            ]),
            1
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::num_magic_squares_inside(vec![vec![8]]), 0);
    }
}
