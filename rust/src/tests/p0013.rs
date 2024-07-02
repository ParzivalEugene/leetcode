#[cfg(test)]
mod test {
    use crate::solutions::p0013::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
