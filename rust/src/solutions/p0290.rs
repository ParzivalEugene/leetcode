use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern_raw: String, s_raw: String) -> bool {
        let pattern = Vec::from_iter(pattern_raw.chars());
        let s = Vec::from_iter(s_raw.split_whitespace());
        let mut word_to_code: HashMap<String, char> = HashMap::new();
        let mut code_to_word: HashMap<char, String> = HashMap::new();

        if s.len() != pattern.len() {
            return false;
        }

        for i in 0..s.len() {
            let code = pattern[i];
            let word = s[i];
            match (word_to_code.get(word), code_to_word.get(&code)) {
                (Some(&table_code), Some(table_word)) => {
                    if code != table_code || word != table_word {
                        return false;
                    }
                }
                (None, None) => {
                    word_to_code.insert(word.to_owned(), code);
                    code_to_word.insert(code, word.to_owned());
                }
                _ => return false,
            }
        }

        true
    }
}
