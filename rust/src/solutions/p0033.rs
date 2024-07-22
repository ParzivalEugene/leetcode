pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return nums
            .iter()
            .position(|&x| x == target)
            .map(|x| x as i32)
            .unwrap_or(-1);
    }
}
