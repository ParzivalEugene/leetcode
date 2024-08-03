pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(mut colors: Vec<i32>) -> i32 {
        colors.extend(colors[..2].to_vec());
        colors
            .windows(3)
            .filter(|group| group[0] != group[1] && group[1] != group[2])
            .count() as i32
    }
}
