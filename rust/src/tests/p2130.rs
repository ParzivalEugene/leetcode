#[cfg(test)]
mod test {
    use crate::solutions::p2130::{new_list, Solution};

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::pair_sum(new_list(vec![5, 4, 2, 1])), 6)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::pair_sum(new_list(vec![4, 2, 2, 3])), 7)
    }
}
