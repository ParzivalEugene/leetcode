#[cfg(test)]
mod test {
    use crate::solutions::p0328::{new_list, Solution};

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::odd_even_list(new_list(vec![1, 2, 3, 4, 5])),
            new_list(vec![1, 3, 5, 2, 4])
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::odd_even_list(new_list(vec![2, 1, 3, 5, 6, 4, 7])),
            new_list(vec![2, 3, 6, 7, 1, 5, 4])
        )
    }
}
