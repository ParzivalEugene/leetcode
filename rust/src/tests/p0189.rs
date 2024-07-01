#[cfg(test)]
mod test {
    use crate::solutions::p0189::Solution;

    #[test]
    fn leetcode_case_1() {
        let (mut nums, k) = (vec![1, 2, 3, 4, 5, 6, 7], 3);
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn leetcode_case_2() {
        let (mut nums, k) = (vec![-1, -100, 3, 99], 2);
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }
}
