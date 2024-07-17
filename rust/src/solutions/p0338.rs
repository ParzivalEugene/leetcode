pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..n + 1)
            .map(|i| {
                format!("{i:b}")
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as i32)
                    .sum()
            })
            .collect::<Vec<i32>>()
    }
}
