#[cfg(test)]
mod test {
    use crate::solutions::p2181::{new_list, Solution};

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::merge_nodes(new_list(vec![0, 3, 1, 0, 4, 5, 2, 0])),
            new_list(vec![0, 3, 1, 0, 4, 5, 2, 0])
        )
    }
}
