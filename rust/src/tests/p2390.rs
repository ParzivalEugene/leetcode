#[cfg(test)]
mod test {
    use crate::solutions::p2390::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::remove_stars(String::from("leet**cod*e")),
            "lecoe".to_string()
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::remove_stars(String::from("erase*****")),
            "".to_string()
        )
    }
}
