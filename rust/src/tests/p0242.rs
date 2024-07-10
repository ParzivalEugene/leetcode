#[cfg(test)]
mod test {
    use crate::solutions::p0242::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::is_anagram(String::from("rat"), String::from("car")),
            false
        )
    }
}
