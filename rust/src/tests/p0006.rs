#[cfg(test)]
mod test {
    use crate::solutions::p0006::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(Solution::convert(String::from("AB"), 1), String::from("AB"))
    }
}
