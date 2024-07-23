pub struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let (s, t) = (s.chars().collect::<Vec<_>>(), t.chars().collect::<Vec<_>>());
        let mut counter: i32 = 0;
        for i in 0..s.len() {
            counter += (i as i32 - t.iter().position(|&x| x == s[i]).unwrap() as i32).abs();
        }
        counter
    }
}
