use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut max_distance = 0;
        let mut current_pos = 0;
        let end = nums.len() - 1;

        for (i, val) in nums.iter().enumerate() {
            if i == end {
                break;
            }

            max_distance = max(max_distance, val + i as i32);

            if current_pos == i as i32 {
                jumps += 1;

                current_pos = max_distance;
            }
        }

        jumps
    }
}
