pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pos = vec![-1; 2];
        for i in 0..nums.len() {
            if nums[i] == target {
                if pos[0].eq(&-1) {
                    pos[0] = i as i32;
                } else {
                    pos[1] = i as i32;
                }
            }
        }
        if pos[0].ne(&-1) && pos[1].eq(&-1) {
            pos[1] = pos[0];
        }
        pos
    }
}
