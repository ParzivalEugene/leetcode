use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut table: HashMap<String, Vec<String>> = HashMap::new();

        for word in strs {
            let mut key_vec: Vec<char> = word.chars().collect();
            key_vec.sort();
            let key = String::from_iter(key_vec);

            table
                .entry(key)
                .and_modify(|words| words.push(word.clone()))
                .or_insert(vec![word]);
        }

        let mut res = Vec::new();
        for group in table.values() {
            res.push(group.clone())
        }
        res
    }
}
