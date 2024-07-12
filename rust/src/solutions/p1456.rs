pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let word = s.chars().collect::<Vec<char>>();
        let (mut count, mut max) = (0, 0);
        let mut i = 0 as usize;
        let len = word.len();
        let vowels = "aeoui".chars().collect::<std::collections::HashSet<_>>();
        while i + k as usize <= len {
            count = 0;
            for j in i..i + k as usize {
                if vowels.contains(&word[j]) {
                    count += 1;
                }
            }
            max = std::cmp::max(max, count);

            i += 1;
        }
        max = std::cmp::max(max, count);
        count = 0;
        for j in len - k as usize..len {
            if vowels.contains(&word[j]) {
                count += 1;
            }
        }
        std::cmp::max(max, count)
    }
}
