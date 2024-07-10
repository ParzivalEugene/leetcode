#[cfg(test)]
mod test {
    use crate::solutions::p0219::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        )
    }
}
