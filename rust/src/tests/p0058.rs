#[cfg(test)]
mod test {
    use crate::solutions::p0058::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::length_of_last_word(String::from("Hello World")),
            5
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::length_of_last_word(String::from("   fly me   to   the moon  ")),
            4
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::length_of_last_word(String::from("luffy is still joyboy")),
            6
        )
    }

    #[test]
    fn leetcode_case_4() {
        assert_eq!(Solution::length_of_last_word(String::from("a")), 1)
    }
}
