#[cfg(test)]
mod test {
    use crate::solutions::p2807::{new_list, Solution};

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(new_list(vec![18, 6, 10, 3])),
            new_list(vec![18, 6, 6, 2, 10, 1, 3])
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(new_list(vec![7])),
            new_list(vec![7])
        );
    }
}
