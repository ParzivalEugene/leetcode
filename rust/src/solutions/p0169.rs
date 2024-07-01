use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter: HashMap<i32, u32> = std::collections::HashMap::new();
        for i in nums {
            if counter.contains_key(&i) {
                *counter.get_mut(&i).unwrap() += 1;
            } else {
                counter.insert(i, 1);
            }
        }
        return counter
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0
            .to_owned();
    }
}
