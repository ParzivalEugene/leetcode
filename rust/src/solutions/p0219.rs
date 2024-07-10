use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut indexes: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..nums.len() {
            indexes
                .entry(nums[i])
                .and_modify(|x| x.push(i))
                .or_insert(vec![i]);
        }
        for group in indexes.values() {
            if group.len() == 1 {
                continue;
            }

            for i in 0..group.len() - 1 {
                if group[i + 1] - group[i] <= k as usize {
                    return true;
                }
            }
        }
        false
    }
}
