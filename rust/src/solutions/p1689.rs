pub struct Solution;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().map(|x| x.to_digit(10).unwrap()).max().unwrap() as i32
    }
}
