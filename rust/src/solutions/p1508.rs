pub struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut sums = vec![];
        for i in 0..n {
            let mut accumulator = 0;
            for j in i..n {
                accumulator += nums[j as usize];
                sums.push(accumulator);
            }
        }
        sums.sort_unstable();
        sums[left as usize - 1..right as usize]
            .iter()
            .fold(0, |acc, x| (x + acc) % 1_000_000_007)
    }
}
