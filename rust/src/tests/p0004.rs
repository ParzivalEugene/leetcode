#[cfg(test)]
mod test {
    use crate::solutions::p0004::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        )
    }
}
