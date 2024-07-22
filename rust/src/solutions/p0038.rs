pub struct Solution;

fn rle(s: &str) -> String {
    let (s, mut res) = (s.chars().collect::<Vec<char>>(), String::new());
    let (mut current, mut counter) = (s[0], 1);
    let (mut i, len) = (1, s.len());
    while i < len {
        if s[i] == current {
            counter += 1;
        } else {
            res.push_str(&format!("{}{}", counter, current));
            counter = 1;
            current = s[i];
        }
        i += 1;
    }
    res.push_str(&format!("{}{}", counter, current));
    res
}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut res = "1".to_string();
        for _ in 1..n {
            res = rle(&res);
        }
        res
    }
}
