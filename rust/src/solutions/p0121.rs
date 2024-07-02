pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut gap = 0;
        for price in prices {
            if min > price {
                min = price;
            }
            let local_gap = price - min;
            if local_gap > gap {
                gap = local_gap;
            }
        }
        return gap;
    }
}