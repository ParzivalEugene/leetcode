pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len();
        let mut dp = vec![0; len + 1];
        for i in 2..len + 1 {
            dp[i] = std::cmp::min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2])
        }
        return dp[len];
    }
}
