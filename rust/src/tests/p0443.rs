#[cfg(test)]
mod test {
    use crate::solutions::p0443::Solution;

    #[test]
    fn leetcode_case_1() {
        let mut vec = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut vec), 6);
        assert_eq!(vec[0..6], vec!['a', '2', 'b', '2', 'c', '3'])
    }

    #[test]
    fn leetcode_case_2() {
        let mut vec = vec!['a'];
        assert_eq!(Solution::compress(&mut vec), 1);
        assert_eq!(vec[0..1], vec!['a'])
    }

    #[test]
    fn leetcode_case_3() {
        let mut vec = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(Solution::compress(&mut vec), 4);
        assert_eq!(vec[0..4], vec!['a', 'b', '1', '2'])
    }
}
