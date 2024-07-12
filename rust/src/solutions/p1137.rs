pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let end = n as usize + 1;
        let mut dp = vec![0; std::cmp::max(end, 3)];
        (dp[0], dp[1], dp[2]) = (0, 1, 1);
        for i in 3..end {
            dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
        }
        dp[end - 1]
    }
}
