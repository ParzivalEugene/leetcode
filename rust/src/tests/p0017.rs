#[cfg(test)]
mod test {
    use crate::solutions::p0017::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        )
    }
}
