pub struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut counter = 0;
        for i in 0..arr.len() {
            let mut value = arr[i];
            for k in i + 1..arr.len() {
                value ^= arr[k];
                if value == 0 {
                    counter += (k - i) as i32;
                }
            }
        }
        counter
    }
}
