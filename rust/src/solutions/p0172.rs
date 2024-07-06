pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        n / 3125 + n / 625 + n / 125 + n / 25 + n / 5
    }
}
