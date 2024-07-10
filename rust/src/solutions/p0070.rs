pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut dp: Vec<i32> = vec![0; n as usize];
        (dp[0], dp[1]) = (1, 2);
        for i in 2..n as usize {
            dp[i] = dp[i - 1] + dp[i - 2]
        }
        dp[(n - 1) as usize]
    }
}
