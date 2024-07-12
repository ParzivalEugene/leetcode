pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let (mut sum_left, mut sum_right) = (vec![0], vec![0]);
        let len = nums.len();
        for i in 0..len - 1 {
            sum_left.push(nums[i] + sum_left[i]);
            sum_right.push(nums[len - i - 1] + sum_right[i])
        }
        sum_right.reverse();
        for i in 0..sum_left.len() {
            if sum_left[i] == sum_right[i] {
                return i as i32;
            }
        }
        -1
    }
}
