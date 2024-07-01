pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut i, mut end) = (0, nums.len());
        while i < end {
            if nums[i] == val {
                nums.remove(i);
                end -= 1;
            } else {
                i += 1;
            }
        }
        return i as i32;
    }
}
