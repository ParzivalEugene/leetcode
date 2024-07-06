pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let (mut result, mut word) = (String::new(), String::new());
        for c in s.trim().chars() {
            if c.is_alphanumeric() {
                word.push(c);
            } else {
                if !word.is_empty() {
                    result.push_str(&word.chars().rev().collect::<String>());
                    result.push(' ');
                }
                word.clear();
            }
        }

        if !word.is_empty() {
            result.push_str(&word.chars().rev().collect::<String>());
            result.push(' ');
        }

        result.trim().chars().rev().collect::<String>()
    }
}
