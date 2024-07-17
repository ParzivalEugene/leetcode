#[cfg(test)]
mod test {
    use crate::solutions::p0394::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::decode_string("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        )
    }
}
