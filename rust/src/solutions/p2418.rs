pub struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, mut heights: Vec<i32>) -> Vec<String> {
        let h = heights.clone();
        heights.sort();
        heights
            .iter()
            .rev()
            .map(|left| names[h.iter().position(|right| left == right).unwrap()].clone())
            .collect::<Vec<String>>()
    }
}
