#[cfg(test)]
mod test {
    use crate::solutions::p0066::Solution;

    #[test]
    fn letcode_case_1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4])
    }

    #[test]
    fn letcode_case_2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2])
    }

    #[test]
    fn letcode_case_3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0])
    }
}
