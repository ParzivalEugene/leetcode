pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut revenue = 0;
        let mut gap = 0;
        let mut trend = false; // false - down, true - up

        for i in 0..prices.len() {
            if trend {
                if prices[i] < prices[i - 1] {
                    trend = false;
                    min = prices[i];
                    revenue += gap;
                } else {
                    gap = prices[i] - min;
                }
            } else {
                if prices[i] >= min {
                    trend = true;
                    gap = prices[i] - min;
                } else {
                    min = prices[i];
                }
            }
        }
        
        if trend {
            revenue += gap;
        }

        return revenue;
    }
}
