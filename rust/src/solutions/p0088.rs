pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        *nums1 = nums1[..m as usize].to_vec();
        nums1.extend(nums2[..n as usize].iter());
        nums1.sort();
    }
}
