pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let last = digits.len() - 1;
        let mut i: usize = 0;
        digits[last] += 1;
        while digits[last - i] == 10 && i < last {
            digits[last - i] = 0;
            digits[last - i - 1] += 1;
            i += 1;
        }
        if digits[0] == 10 {
            digits[0] = 0;
            digits.insert(0, 1);
        }
        digits
    }
}
