use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: Vec<char> = Vec::new();
        for i in 0..strs.iter().min().unwrap().len() {
            let mut chars: HashSet<char> = HashSet::new();
            for s in strs.iter() {
                chars.insert(s.chars().nth(i).unwrap());
            }
            if chars.len() != 1 {
                return String::from_iter(prefix.iter());
            } else {
                prefix.push(*chars.iter().next().unwrap())
            }
        }
        String::from_iter(prefix.iter())
    }
}
