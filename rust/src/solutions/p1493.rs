pub struct Solution;

impl Solution {
    pub fn longest_subarray(mut nums: Vec<i32>) -> i32 {
        nums = nums
            .split(|&x| x == 0)
            .map(|x| x.len() as i32)
            .collect::<Vec<i32>>();
        match nums.len() == 1 {
            true => nums[0] - 1,
            false => nums
                .windows(2)
                .map(|window| window[0] + window[1])
                .max()
                .unwrap(),
        }
    }
}
