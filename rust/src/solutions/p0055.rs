pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();

        if len == 1 {
            return true;
        };

        let mut dp = vec![0; len];
        dp[0] = nums[0];

        for i in 1..(len - 1) {
            if dp[i - 1] < (i as i32) {
                return false;
            };
            dp[i] = std::cmp::max(dp[i - 1], nums[i] + (i as i32));
            if dp[i] >= (len - 1) as i32 {
                return true;
            }
        }

        return dp[len - 2] >= (len - 1) as i32;
    }
}
