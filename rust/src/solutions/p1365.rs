pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 101];
        for &num in &nums {
            count[num as usize] += 1;
        }

        let mut sum = 0;
        let mut sum_count = vec![0; 101];
        for i in 0..101 {
            sum_count[i] = sum;
            sum += count[i];
        }

        let mut result = vec![0; nums.len()];
        for i in 0..nums.len() {
            result[i] = sum_count[nums[i] as usize];
        }

        result
    }
}
