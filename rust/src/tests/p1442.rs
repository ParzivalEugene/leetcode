#[cfg(test)]
mod test {
    use crate::solutions::p1442::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10)
    }
}
