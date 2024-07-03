pub struct Solution;

impl Solution {
    pub fn is_palindrome(line: String) -> bool {
        let s: Vec<char> = line.to_lowercase().chars().collect();
        let (mut left, mut right): (usize, usize) = (0, s.len() - 1);
        while left < right {
            if !s[left].is_alphanumeric() {
                left += 1;
            } else if !s[right].is_alphanumeric() {
                right -= 1;
            } else if s[left] == s[right] {
                left += 1;
                right -= 1;
            } else {
                println!("{} {}", s[left], s[right]);
                return false;
            }
        }
        return true;
    }
}
