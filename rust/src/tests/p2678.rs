#[cfg(test)]
mod test {
    use crate::solutions::p2678::Solution;

    #[test]
    fn leetcode_case_1() {
        let details = vec![
            "7868190130M7522".to_string(),
            "5303914400F9211".to_string(),
            "9273338290F4010".to_string(),
        ];
        assert_eq!(Solution::count_seniors(details), 2);
    }

    #[test]
    fn leetcode_case_2() {
        let details = vec!["1313579440F2036".to_string(), "2921522980M5644".to_string()];
        assert_eq!(Solution::count_seniors(details), 0);
    }
}
