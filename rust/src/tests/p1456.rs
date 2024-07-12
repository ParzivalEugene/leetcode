#[cfg(test)]
mod test {
    use crate::solutions::p1456::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::max_vowels(String::from("abciiidef"), 3), 3)
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::max_vowels(String::from("aeiou"), 2), 2)
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::max_vowels(String::from("leetcode"), 3), 2)
    }

    #[test]
    fn leetcode_case_4() {
        assert_eq!(Solution::max_vowels(String::from("weallloveyou"), 7), 4)
    }
}
