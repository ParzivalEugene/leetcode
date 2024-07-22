pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.chars()
            .collect::<Vec<char>>()
            .windows(2)
            .map(|slice| (slice[0] as i32 - slice[1] as i32).abs())
            .sum()
    }
}
