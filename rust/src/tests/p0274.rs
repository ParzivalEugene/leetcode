#[cfg(test)]
mod test {
    use crate::solutions::p0274::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::h_index(vec![99, 100]), 2);
    }

    #[test]
    fn leetcode_case_4() {
        assert_eq!(Solution::h_index(vec![99, 100, 0, 0]), 2);
    }

    #[test]
    fn leetcode_case_5() {
        assert_eq!(Solution::h_index(vec![1, 7, 9, 4]), 3);
    }
}
