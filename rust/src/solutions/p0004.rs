pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort();
        let mid = nums1.len() / 2;
        if nums1.len() % 2 == 0 {
            (nums1[mid - 1] + nums1[mid]) as f64 / 2.0
        } else {
            nums1[mid] as f64
        }
    }
}
