pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let arr_size = nums.len();
        let (mut start, mut end) = (0, 0);
        while end < arr_size {
            if nums[end] == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[start] == 0 {
                    k += 1;
                }
                start += 1;
            }
            end += 1;
        }

        (end - start) as i32
    }
}
