pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut counter = 0;
        for c in stones.chars() {
            if jewels.contains(c) {
                counter += 1;
            }
        }
        counter
    }
}
