use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        return Self::mask(&s).eq(&Self::mask(&t));
    }

    pub fn mask(s_raw: &str) -> Vec<u32> {
        let s = Vec::from(s_raw);
        let mut marks: HashMap<u8, u32> = HashMap::new();
        let (len, mut current): (usize, u32) = (s.len(), 0);
        marks.insert(s[0], current);
        let mut masked = vec![current];

        for i in 1..len {
            if s[i] != s[i - 1] {
                if !marks.contains_key(&s[i]) {
                    current += 1;
                    marks.insert(s[i], current);
                }
            }
            masked.push(marks.get(&s[i]).unwrap().clone());
        }
        masked
    }
}
