pub struct Solution;

impl Solution {
    fn gcd(mut n: usize, mut m: usize) -> usize {
        while m != 0 {
            if m < n {
                std::mem::swap(&mut m, &mut n);
            }
            m %= n;
        }
        n
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if (str1.to_owned() + &str2 != str2.to_owned() + &str1) {
            return String::new();
        }

        let len = Self::gcd(str1.len(), str2.len());
        return str1[0..len].to_owned();
    }
}
