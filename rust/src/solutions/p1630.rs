pub struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut result = vec![];

        for (&start, &end) in l.iter().zip(r.iter()) {
            let mut subarray: Vec<i32> = nums[start as usize..=end as usize].to_vec();
            subarray.sort_unstable();
            let diff = subarray[1] - subarray[0];
            let is_arithmetic = subarray.windows(2).all(|w| w[1] - w[0] == diff);
            result.push(is_arithmetic);
        }

        result
    }
}
