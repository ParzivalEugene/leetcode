use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut visited: HashSet<i32> = HashSet::new();
        while !visited.contains(&n) {
            visited.insert(n);
            n = n
                .to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap().pow(2) as i32)
                .sum();
            if n == 1 {
                return true;
            }
        }
        false
    }
}
