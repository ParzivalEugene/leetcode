pub struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        for asteroid in asteroids {
            stack.push(asteroid);
            while stack.len() > 1 && stack[stack.len() - 2] > 0 && stack[stack.len() - 1] < 0 {
                let l = stack.pop().unwrap();
                let r = stack.pop().unwrap();
                match r.cmp(&-l) {
                    std::cmp::Ordering::Less => stack.push(l),
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => stack.push(r),
                }
            }
        }
        stack
    }
}
