#[cfg(test)]
mod test {
    use crate::solutions::p0049::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::group_anagrams(vec![String::new()]),
            vec![vec![String::new()]]
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::group_anagrams(vec![String::from("a")]),
            vec![vec![String::from("a")]]
        );
    }

    #[test]
    fn leetcode_case_3() {
        Solution::group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ]);
    }
}
