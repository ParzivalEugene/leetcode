#[cfg(test)]
mod test {
    use crate::solutions::p2352::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ]),
            3
        )
    }
}
