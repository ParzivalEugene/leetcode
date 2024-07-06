pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let data = s.trim().chars().rev().collect::<Vec<char>>();
        for i in 0..data.len() {
            if !data[i].is_alphanumeric() {
                return i as i32;
            }
        }
        data.len() as i32
    }
}
