#[cfg(test)]
mod test {
    use crate::solutions::p0205::Solution;

    #[test]
    fn leetocode_case_1() {
        assert_eq!(
            Solution::is_isomorphic(String::from("egg"), String::from("add")),
            true
        );
    }

    #[test]
    fn leetocode_case_2() {
        assert_eq!(
            Solution::is_isomorphic(String::from("foo"), String::from("bar")),
            false
        );
    }

    #[test]
    fn leetocode_case_3() {
        assert_eq!(
            Solution::is_isomorphic(String::from("paper"), String::from("title")),
            true
        );
    }

    #[test]
    fn leetocode_case_4() {
        assert_eq!(
            Solution::is_isomorphic(String::from("badc"), String::from("baba")),
            false
        );
    }
}
