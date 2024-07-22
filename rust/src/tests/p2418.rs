#[cfg(test)]
mod test {
    use crate::solutions::p2418::Solution;

    fn vec(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::sort_people(vec(vec!["Mary", "John", "Emma"]), vec![180, 165, 170]),
            vec!["Mary", "Emma", "John"]
        );
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::sort_people(
                vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
                vec![155, 185, 150]
            ),
            vec!["Bob", "Alice", "Bob"]
        );
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::sort_people(
                vec(vec!["A", "B", "C", "D", "E", "F", "G"]),
                vec![1, 10, 2, 3, 11, 23, 123]
            ),
            vec!["G", "F", "E", "B", "D", "C", "A"]
        )
    }
}
