pub struct Solution;

impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let (mut i, mut j) = (0 as usize, nums.len() - 1);
        let mut counter = 0;
        nums.sort();
        while i < j {
            let sum = nums[i] + nums[j];
            if sum == k {
                counter += 1;
                i += 1;
                j -= 1;
            } else if sum > k {
                j -= 1;
            } else {
                i += 1
            }
        }
        counter
    }
}
