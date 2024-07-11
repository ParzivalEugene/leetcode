pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut pools: Vec<String> = vec![String::new(); num_rows as usize];
        let (mut i, mut dir) = (0, 1);
        let end = num_rows - 1;
        for c in s.chars() {
            pools[i as usize].push(c);
            if i == end && dir > 0 || i == 0 && dir < 0 {
                dir = -dir;
            }
            i += dir;
        }
        return pools.join("");
    }
}
