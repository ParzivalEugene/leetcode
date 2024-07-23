pub struct Solution;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let counts = nums
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, &num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            });

        nums.sort_by_cached_key(|&num| (counts[&num], std::cmp::Reverse(num)));
        nums
    }
}
