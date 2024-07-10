#[cfg(test)]
mod test {
    use crate::solutions::p0071::Solution;

    #[test]
    fn leetcode_case_1() {
        assert_eq!(
            Solution::simplify_path(String::from("/home/")),
            String::from("/home")
        )
    }

    #[test]
    fn leetcode_case_2() {
        assert_eq!(
            Solution::simplify_path(String::from("/home//foo")),
            String::from("/home/foo")
        )
    }

    #[test]
    fn leetcode_case_3() {
        assert_eq!(
            Solution::simplify_path(String::from("/home/user/Documents/../Pictures")),
            String::from("/home/user/Pictures")
        )
    }

    #[test]
    fn leetcode_case_4() {
        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        )
    }

    #[test]
    fn leetcode_case_5() {
        assert_eq!(
            Solution::simplify_path(String::from("/.../a/../b/c/../d/./")),
            String::from("/.../b/d")
        )
    }
}
