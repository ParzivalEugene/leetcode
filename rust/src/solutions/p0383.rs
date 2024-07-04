use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn counter(string: &str) -> HashMap<char, i32> {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in string.chars() {
            counter.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }
        counter
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters = Solution::counter(&ransom_note);
        for c in magazine.chars() {
            if letters.contains_key(&c) {
                letters
                    .entry(c)
                    .and_modify(|c| *c = std::cmp::max(*c - 1, 0));
            }
        }
        return letters.values().all(|&x| x == 0);
    }
}
