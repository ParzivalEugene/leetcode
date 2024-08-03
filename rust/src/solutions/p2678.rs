pub struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter(|person| person[11..13].parse::<u32>().unwrap() > 60)
            .count() as i32
    }
}
