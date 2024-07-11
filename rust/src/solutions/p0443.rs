pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut count = 1;
        let mut last = chars[0];
        let mut res: Vec<char> = vec![];
        for i in 1..chars.len() {
            if chars[i] != last {
                res.push(last);
                if count != 1 {
                    res.append(&mut count.to_string().chars().collect::<Vec<char>>())
                }
                count = 1;
                last = chars[i];
            } else {
                count += 1;
            }
        }
        res.push(last);
        if count != 1 {
            res.append(&mut count.to_string().chars().collect::<Vec<char>>());
        }
        std::mem::swap(chars, &mut res);
        return chars.len() as i32;
    }
}
