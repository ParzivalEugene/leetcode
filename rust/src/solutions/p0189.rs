pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let last = nums.len() - 1;
        for _ in 0..k {
            nums.insert(0, nums[last]);
            nums.pop();
        }
    }
}
