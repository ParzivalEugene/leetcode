pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut pre2 = 0;
        let mut pre1 = 0;
        for num in nums {
            let next_pre1 = std::cmp::max(num + pre2, pre1);
            pre2 = pre1;
            pre1 = next_pre1;
        }
        pre1
    }
}
