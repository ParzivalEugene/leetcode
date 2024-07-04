#[cfg(test)]
mod test {
    use crate::solutions::p0383::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::can_construct(String::from("a"), String::from("b")),
            false
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::can_construct(String::from("aa"), String::from("ab")),
            false
        );
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::can_construct(String::from("aa"), String::from("aab")),
            true
        );
    }

    #[test]
    fn leetcode_case_4() {
        assert_eq!(
            Solution::can_construct(
                String::from("bg"),
                String::from("efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj")
            ),
            true
        );
    }
}
