pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut ranges: Vec<String> = vec![];
        let (mut start, mut end) = (nums[0], nums[0]);
        for i in 0..nums.len() - 1 {
            if nums[i] + 1 == nums[i + 1] {
                end = nums[i + 1]
            } else {
                if start == end {
                    ranges.push(nums[i].to_string())
                } else {
                    ranges.push(format!("{}->{}", start, end))
                }
                start = nums[i + 1];
                end = nums[i + 1];
            }
        }
        if start == end {
            ranges.push(start.to_string());
        } else {
            ranges.push(format!("{}->{}", start, end))
        }
        ranges
    }
}
