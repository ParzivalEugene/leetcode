pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = Vec::from(s);
        let roman = HashMap::from([
            ('M', 1000),
            ('D', 500),
            ('C', 100),
            ('L', 50),
            ('X', 10),
            ('V', 5),
            ('I', 1),
        ]);
        let mut res = 0;
        for i in 0..s.len() - 1 {
            if roman[&(s[i] as char)] < roman[&(s[i + 1] as char)] {
                res -= roman[&(s[i] as char)]
            } else {
                res += roman[&(s[i] as char)]
            }
        }
        return res + roman[&(s[s.len() - 1] as char)];
    }
}
