#[cfg(test)]
mod test {
    use crate::solutions::p0088::Solution;

    #[test]
    fn leetcode_case_1() {
        let (mut nums1, mut nums2, m, n) = (vec![2, 5, 6], vec![1, 2, 3, 0, 0, 0], 3, 3);
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(&nums1[..m as usize], vec![1, 2, 2]);
    }

    #[test]
    fn leetcode_case_2() {
        let (mut nums1, mut nums2, m, n) = (vec![1], vec![], 1, 0);
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(&nums1[..m as usize], vec![1]);
    }

    #[test]
    fn leetcode_case_3() {
        let (mut nums1, mut nums2, m, n) = (vec![0], vec![1], 0, 1);
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(&nums1[..m as usize], vec![]);
    }
}
