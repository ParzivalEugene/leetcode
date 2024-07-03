pub struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut odd_counter = 0;
        for i in arr {
            if odd_counter > 2 {
                return true;
            }
            if i % 2 == 1 {
                odd_counter += 1;
            } else {
                odd_counter = 0
            }
        }
        return odd_counter > 2;
    }
}
