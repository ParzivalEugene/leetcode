#[cfg(test)]
mod test {
    use crate::solutions::p0014::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            String::from("fl")
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car"),
            ]),
            String::from("")
        )
    }
}
