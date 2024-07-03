pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut s, mut e): (usize, usize) = (0, height.len() - 1);
        let mut max_area = 0;
        while e - s >= 1 {
            let area = std::cmp::min(height[s], height[e]) as usize * (e - s);
            if area > max_area {
                max_area = area;
            }
            if height[s] > height[e] {
                e -= 1;
            } else {
                s += 1;
            }
        }
        max_area as i32
    }
}
