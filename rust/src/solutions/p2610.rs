pub struct Solution;

impl Solution {
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![nums[0]]];
        nums.remove(0);
        for num in nums {
            let mut added = false;
            for i in 0..matrix.len() {
                if !matrix[i].contains(&num) {
                    matrix[i].push(num);
                    added = true;
                    break;
                }
            }
            if !added {
                matrix.push(vec![num]);
            }
        }
        matrix
    }
}
