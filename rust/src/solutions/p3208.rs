pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let (mut result, mut prev, mut count) = (0, colors[0], 1);
        for idx in 1..colors.len() {
            count = if colors[idx] != prev { count + 1 } else { 1 };
            if count >= k {
                result += 1
            }
            prev = colors[idx]
        }
        for idx in 0..k as usize - 1 {
            count = if colors[idx] != prev { count + 1 } else { 1 };
            if count >= k {
                result += 1
            }
            prev = colors[idx]
        }
        result
    }
}
