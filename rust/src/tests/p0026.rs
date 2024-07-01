#[cfg(test)]
mod test {
    use crate::solutions::p0026::Solution;

    #[test]
    fn leetcode_case_1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    }

    #[test]
    fn leetcode_case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    }
}
