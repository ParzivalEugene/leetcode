use std::collections::{HashMap, HashSet};
pub struct Solution;

impl Solution {
    fn count(word: &str) -> Vec<u32> {
        let mut counter: HashMap<char, u32> = HashMap::new();
        for c in word.chars() {
            counter.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut values = counter.values().map(|x| *x).collect::<Vec<u32>>();
        values.sort();
        values
    }

    pub fn close_strings(word1: String, word2: String) -> bool {
        let (chars1, chars2) = (
            word1.chars().collect::<HashSet<_>>(),
            word2.chars().collect::<HashSet<_>>(),
        );
        let (values1, values2) = (Self::count(&word1), Self::count(&word2));
        return chars1 == chars2 && values1 == values2;
    }
}
