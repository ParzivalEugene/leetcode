#[cfg(test)]
mod test {
    use crate::solutions::p0021::{new_list, Solution};

    #[test]
    fn leetcode_case_1() {
        let (list1, list2) = (new_list(vec![1, 2, 4]), new_list(vec![1, 3, 4]));
        let result = new_list(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), result);
    }

    #[test]
    fn leetcode_case_2() {
        let (list1, list2) = (new_list(vec![]), new_list(vec![]));
        let result = new_list(vec![]);
        assert_eq!(Solution::merge_two_lists(list1, list2), result);
    }

    #[test]
    fn leetcode_case_3() {
        let (list1, list2) = (new_list(vec![]), new_list(vec![0]));
        let result = new_list(vec![0]);
        assert_eq!(Solution::merge_two_lists(list1, list2), result);
    }
}
