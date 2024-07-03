pub struct Solution;

impl Solution {
    pub fn is_subsequence(input_s: String, input_t: String) -> bool {
        let (s, t): (Vec<char>, Vec<char>) = (input_s.chars().collect(), input_t.chars().collect());
        let (mut i, mut j): (usize, usize) = (0, 0);
        let (s_len, t_len) = (s.len(), t.len());
        while j < t_len && i < s_len {
            if s[i] == t[j] {
                i += 1;
            }
            j += 1;
        }
        return i == s_len;
    }
}
