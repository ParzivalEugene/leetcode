use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counter: HashMap<char, u32> = HashMap::new();
        for c in s.chars() {
            counter.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
        for c in t.chars() {
            match counter.get(&c) {
                Some(&x) => match x == 1 {
                    true => {
                        counter.remove(&c);
                    }
                    false => {
                        counter.insert(c, x - 1);
                    }
                },
                _ => return false,
            }
        }
        true
    }
}
