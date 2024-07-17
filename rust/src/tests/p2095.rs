#[cfg(test)]
mod test {
    use crate::solutions::p2095::{new_list, Solution};

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::delete_middle(new_list(vec![1, 3, 4, 7, 1, 2, 6])),
            new_list(vec![1, 3, 4, 1, 2, 6])
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::delete_middle(new_list(vec![1, 2, 3, 4])),
            new_list(vec![1, 2, 4])
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::delete_middle(new_list(vec![2, 1])),
            new_list(vec![2])
        )
    }
}
