#[cfg(test)]
mod test {
    use crate::solutions::p0338::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1])
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2])
    }
}
