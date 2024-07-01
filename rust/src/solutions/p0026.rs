pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut i, mut end) = (0, nums.len());
        let mut entries = vec![];
        while i < end {
            if entries.contains(&nums[i]) {
                nums.remove(i);
                end -= 1;
            } else {
                entries.push(nums[i]);
                i += 1;
            }
        }
        return i as i32;
    }
}
