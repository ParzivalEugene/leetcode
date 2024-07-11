#[cfg(test)]
mod test {
    use crate::solutions::p1071::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("ABCABC"), String::from("ABC")),
            String::from("ABC")
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("ABABAB"), String::from("ABAB")),
            String::from("AB")
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("LEET"), String::from("CODE")),
            String::from("")
        )
    }
}
