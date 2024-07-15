#[cfg(test)]
mod test {
    use crate::solutions::p1493::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2)
    }

    #[test]
    fn leetcode_case_4() {
        assert_eq!(Solution::longest_subarray(vec![0, 0, 1, 1]), 2)
    }

    #[test]
    fn leetcode_case_5() {
        assert_eq!(
            Solution::longest_subarray(vec![1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1]),
            11
        )
    }
}
