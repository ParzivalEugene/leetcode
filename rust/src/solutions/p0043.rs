pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let (len1, len2) = (num1.len(), num2.len());
        let mut digits = vec![0; len1 + len2];

        for (i1, c1) in num1.chars().rev().enumerate() {
            for (i2, c2) in num2.chars().rev().enumerate() {
                let res = c1.to_digit(10).unwrap() * c2.to_digit(10).unwrap() + digits[i1 + i2];
                digits[i1 + i2] = res % 10;
                digits[i1 + i2 + 1] += res / 10;
            }
        }

        let res = digits
            .iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<String>()
            .trim_start_matches(|x| x == '0')
            .to_owned();

        match res.is_empty() {
            true => "0".to_owned(),
            _ => res,
        }
    }
}
