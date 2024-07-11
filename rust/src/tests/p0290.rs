#[cfg(test)]
mod test {
    use crate::solutions::p0290::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat dog")),
            true
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::word_pattern(String::from("abba"), String::from("dog cat cat fish")),
            false
        );
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::word_pattern(String::from("aaaa"), String::from("dog cat cat dog")),
            false
        );
    }

    #[test]
    fn leetcode_case_4() {
        assert_eq!(
            Solution::word_pattern(String::from("aaaa"), String::from("dog dog dog dog")),
            true
        );
    }

    #[test]
    fn leetcode_case_5() {
        assert_eq!(
            Solution::word_pattern(String::from("a"), String::from("dog")),
            true
        );
    }
}
