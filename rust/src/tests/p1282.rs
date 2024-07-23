#[cfg(test)]
mod test {
    use crate::solutions::p1282::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
            vec![vec![1], vec![0, 5], vec![2, 3, 4]]
        )
    }
}
