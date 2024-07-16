pub struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = String::new();
        for c in s.chars() {
            match c {
                '*' => {
                    stack.pop();
                }
                _ => {
                    stack.push(c);
                }
            }
        }
        stack
    }
}
